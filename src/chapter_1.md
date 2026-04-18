# Chapter 1

## Where Traditional Queries Fail
Picture this: your dog just birthed a cute litter of say 5 puppies.

Now let's assume you have no other pets.

If I asked you: "how many pets do you have?", you would naturally say 6.
If I asked you: "how many dogs do you have?", you would of course still say 6.

Now I asked slightly similar questions, but you were able to parse the question and answer it correctly.

If you were querying a database for the same answer you would write something to this effect:

```sql
SELECT * FROM animals WHERE type = 'pet';
```
or

```sql
SELECT * FROM pets WHERE type = 'dog';
```

Depending on how you designed your database, your query might yield your desired result.

But we're already running into some common problems. Chief of them is that we need to design our tables and queries to accurately represent relationships between data — not a small feat in itself. For that we need structured data.

Bear in mind that most everyday data is unstructured:

- "I have 2 Rottweilers"
- "I have 3 Cats"
- "I have 3 Tom cats"
- "I have a pet parrot"
- "I have a pet bird"

To us humans, all the above questions obviously have high "similarity".

But in a database there's no implicit similarity between them — it treats each piece of data as an isolated point — unless we make any such relationship explicit somehow. That leads us to the most important issue: encoding such "similarities" to enable computers to process them.

## Vector Search

Vectors provide a comprehensive mathematical encoding of the similarities between such unstructured data.

In its simplest form, a vector is any quantity that has both magnitude and direction.

Visualize the diagram in the image below.

![Alt text](img_1.png)

Imagine you're standing at the origin (the center point marked "0").

I could tell you to move **60 km**, but that is incomplete — 60km *where*? you ask.

You need both pieces of information:
- **Magnitude:** 60 km (how far)
- **Direction:** Northwest (which way)

So the complete instruction is: "Move 60km Northwest" or "Move 40km Southeast" or "Move 40km North."

### Why Direction Matters
Disclaimer: I personally suck at compass navigation.
Now imagine three people starting at the origin:
- **Person A:** Moves 90km Northwest
- **Person B:** Moves 60km Northwest
- **Person C:** Moves 10km Southeast

Even though Person C traveled the least (10km), they're actually the **farthest** from Person A.

Person B, who only went 60km, is **closer** to Person A than Person C is.

Why? Person B's direction (Northwest) is more similar to Person A's direction (Northwest) than Person C's direction (Southeast). Hence both their travel destinations did not diverge as far as Person B and Person C's.

### Measuring Similarity with Angles
But the previous instruction is itself still not complete.

Northwest could be any direction between North and West; to be more precise we could add angles:

- move 90km 85° Northwest
- move 60km 60° Northwest
- move 10km 230° Southeast

![Alt text](img_2.png)

How do we measure how "similar" any two directions are?
We measure the **angle** between them.

![Alt text](img_3.png)

The smaller the angle, the more similar the two directions are and the lesser the divergence as we travel outwards from the origin.
If we represent "I have a dog" as a vector at 40°, "My dog gave birth to a litter of five puppies" as 60°, and "Car" as 230°, we develop a system for ranking similarity. Crucially, this is magnitude-invariant. Just because "A" is a whole book about dogs doesn't change its similarity to the sentence "I have a dog," because they both point in the same "thematic" direction.

## Back to Queries
Back to our cute puppy litters.

If we could represent each sentence as a vector:
- "How many pets do you have?" → Vector A (pointing in some direction)
- "How many dogs do you have?" → Vector B (pointing in a similar direction)
- "What's the weather today?" → Vector C (pointing in a completely different direction)

Then finding similar questions becomes a simple matter of:
- Converting and labeling our data as vectors
- Converting the query into a vector
- Measuring the angle between the query and all stored vectors
- Returning the ones with the smallest angles (and consequently higher similarity)

By encoding data into a mathematical representation on a vector plane, 
we have achieved a means of comparing data points to one another 
across various dimensions.
But the most important measure we want here is cosine similarity.