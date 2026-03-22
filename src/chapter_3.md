# Refactoring

In the last chapter we managed to cobble up something working from scratch, but 
it's not enough to just work — we must make it better.

In this chapter, we're going to poke some holes in our previous setup and look 
at how we can make it more secure and robust.

Let's start with our method of creating embeddings. As of now we're tied to 
using Gemini for generating embeddings. With the host of different embedding 
models and providers out there, there's no reason we should handicap ourselves 
that way — let's change that.

## Introducing Rig

Rig is the go-to crate for dealing with LLMs and agents in Rust. From its 
official docs:

> A modular, scalable library for building LLM-powered applications. 
> Production-ready and open source.

Our current approach for generating vectors looks like this:
```rust
{{#include ../embeddings/src/types.rs:8:29}}
```
```rust
{{#include ../embeddings/src/types.rs:109:136}}
```

Our only way of converting text to vectors is by making an HTTP request to a 
Gemini endpoint and deserializing the response into a Gemini-specific struct — 
Gemini-specific being the problem.

Rig provides a unified abstraction over multiple embedding providers and models 
through its `EmbeddingModel` trait:
```rust
pub trait EmbeddingModel {
    fn embed_text(&self, text: &str) 
        -> impl Future<Output = Result<Embedding, EmbeddingError>>;
    
    fn embed_texts(&self, texts: impl IntoIterator<Item = String>) 
        -> impl Future<Output = Result<Vec<Embedding>, EmbeddingError>>;
}
```

Any provider that implements this trait — OpenAI, Gemini, Cohere, whatever — 
works identically from our code's perspective. To switch from Gemini to OpenAI, 
you change one line. That's the abstraction we want.

Better yet — we don't need our own `Embedding` struct at all. Rig ships with 
one:
```rust
pub struct Embedding {
    pub document: String,
    pub vec: Vec<f64>,
}
```

Compare that to what we had:
```rust
pub struct Embedding {
    pub label: String,
    pub vector: Vec<f32>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    embedding: EmbeddingValues,
}

#[derive(Deserialize)]
struct BatchGeminiResponse {
    embeddings: Vec<EmbeddingValues>,
}

#[derive(Deserialize)]
struct EmbeddingValues {
    values: Vec<f64>,
}
```

All of that — plus `vectorize()`, `batch_vectorize()`, `new()`, `batch_new()`, 
and `create_client()` — is now handled by Rig. We just delete it.

## The Refactored types.rs

What survives from Chapter 2 is exactly what should survive — the logic that 
is genuinely ours: storing vectors, retrieving them, and computing distance.
```rust
{{#include ../refactored/src/types.rs}}
```

`types.rs` went from ~180 lines to ~60. Every deleted line was boilerplate 
that Rig now owns. What remains is focused and clear.

Two things worth noting:

The functions are now standalone rather than methods on an `Embedding` impl 
block — since we're using Rig's `Embedding` type directly, there's no struct 
of our own to hang methods on.

The vectors are now `f64` throughout, matching Rig's `Embedding.vec` type. 
In Chapter 2 we used `f32` — the cast on the way in and out of SQLite is still 
there but it's consolidated.

## The Refactored main.rs

The other big change is in `main.rs` — we now pass a `model` around instead 
of a `client`:
```rust
{{#include ../refactored/src/main.rs}}
```

In Chapter 2, `main` created a reqwest client and passed it everywhere. 
Functions called `Embedding::new()` which called `vectorize()` which made the 
HTTP request. The embedding logic was buried three layers deep.

Now `main` creates a model and passes it directly to `load_faq` and 
`search_faq`. Those functions call `model.embed_text()` — one call, no layers.

And if you want to switch providers:
```rust
// Gemini
let client = gemini::Client::from_env();
let model = client.embedding_model(gemini::GEMINI_EMBEDDING_001);

// OpenAI — swap these two lines, everything else stays the same
let client = openai::Client::from_env();
let model = client.embedding_model(openai::TEXT_EMBEDDING_ADA_002);
```

That's it. The rest of the codebase doesn't change.

## What We Deleted

It's worth being explicit about what's gone:

| Removed | Why |
|---|---|
| `GeminiResponse` struct | Rig handles deserialization |
| `BatchGeminiResponse` struct | Rig handles deserialization |
| `EmbeddingValues` struct | Rig handles deserialization |
| `create_client()` | Rig manages the HTTP client |
| `vectorize()` | Replaced by `model.embed_text()` |
| `batch_vectorize()` | Replaced by `model.embed_texts()` |
| `new()` | No longer needed |
| `batch_new()` | No longer needed |
| `Embedding` struct | Using `rig::embeddings::Embedding` |
| `reqwest` dependency | Rig handles HTTP |
| `serde` / `serde_json` dependencies | Rig handles serialization |

Good refactoring is measured in deletions as much as additions. We deleted 
~120 lines and the program does exactly the same thing — just with less to 
maintain and no provider lock-in.

## What's Next

Our search logic is solid and our embedding layer is now clean and swappable. 
But we're still missing proper error handling — if anything goes wrong, 
we crash. In the next chapter we'll fix that, giving our program the robustness 
it needs before we wire it up to a Telegram bot.