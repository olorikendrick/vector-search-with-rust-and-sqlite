# Vector Search from Scratch: Building a FAQ Chatbot

You might have wondered: how does Google know to show you exactly what you 
need even when you type something slightly different?

Meanwhile, your SQL queries return null when even one word is off.

The answer is **Vector search**.

You might have used it before when building RAG systems or working with AI 
tools, but do you actually know how it works?

"Semantics" is all about meaning and context — and here you'll learn how to 
use vectors to encode them in a way that computers can understand so we can 
make the best use of it.

In this guide, we'll build a fully functional vector search system from 
scratch (mostly lol), and by the end, we'll hook it up to a chatbot for RAG.

## What You'll Build

By the end of this guide, you'll have:

- A vector search engine that understands meaning
- An FAQ chatbot powered by vector embeddings
- A working RAG (Retrieval-Augmented Generation) system
- Deep understanding of how it all works

Although we'll build our system with SQLite, the patterns, architecture, and 
design principles you'll learn here are not SQLite-specific. They apply to 
any vector search system you want to build — whether you later move to 
Postgres with pgvector, specialized vector databases, or even in-memory 
solutions.

SQLite is a solid choice for this because it has such a low resource 
footprint and negligible setup cost. It exists on most devices, so that's a 
plus if you ever want to deploy such an app.

## Who This Guide Is For

This guide assumes you already have:

- Basic programming skills in any language
- Some familiarity with databases (SQL basics)
- Curiosity about how AI search works

The code is in Rust. You can follow the concepts without knowing Rust — but 
to implement along with the examples you'll need basic Rust familiarity, 
particularly around structs, traits, and serialization with Serde. The 
[Rust Book](https://doc.rust-lang.org/book/) is the best place to get up 
to speed.

## Setup

```bash
git clone https://github.com/olorikendrick/vector-search-with-rust-and-sqlite
cd vector-search-with-rust-and-sqlite/embeddings
```

Each chapter may introduce breaking changes. To follow along cleanly, 
we encourage you to branch per chapter:

```bash
git checkout -b chapter-2
cargo run
```

Optionally replace `faq.txt` with your own FAQ file before running.

## Chapters

**[Chapter 1: Where Traditional Queries Fail](https://olorikendrick.github.io/vector-search-with-rust-and-sqlite/chapter_1.html)**  
We explore the limits of traditional SQL and what we can use in its place.

**[Chapter 2: Building Vector Search](https://olorikendrick.github.io/vector-search-with-rust-and-sqlite/chapter_2.html)**  
Generate embeddings, store them in SQLite, calculate cosine similarity, 
and search.

**[Chapter 3: Refactoring](https://olorikendrick.github.io/vector-search-with-rust-and-sqlite/chapter_3.html)**  
Clean up the codebase and 
add proper error handling.

**Chapter 4: The FAQ Chatbot** *(coming soon)*  
Hook it all up to build a complete RAG system.

---

Any support, contributions, or corrections are appreciated — it keeps me going.

If you find it helpful, star and share!