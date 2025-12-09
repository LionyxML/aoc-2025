## DISJOINT SET UNION (DSU) / UNION–FIND – TEXT EXPLANATION

---

1. WHAT IS A DSU?

---

A DSU (Disjoint Set Union), also called Union-Find, is a data structure
that keeps track of a collection of elements partitioned into groups
(sets) that do not overlap.

It supports two main operations:

* FIND(x): Determine which group (set) element x belongs to.
* UNION(x, y): Merge the group containing x with the group containing y.

The DSU efficiently maintains a "forest" of trees representing these sets.

The typical performance is almost O(1) per operation (amortized), thanks
to two techniques:

* Path Compression
* Union by Rank / Size

---

2. WHY DSU EXISTS — WHAT PROBLEMS IT SOLVES

---

DSU is perfect for problems where you repeatedly connect things and need
to check if two things are connected.

Common uses:

(1) Connectivity in graphs
- "Are nodes A and B in the same connected component?"
- "Add an edge and merge two components."

(2) Kruskal’s Minimum Spanning Tree algorithm
- Repeatedly check whether adding an edge would create a cycle.

(3) Dynamic grouping
- Track friend groups, computer networks, social clusters, islands, etc.

(4) Cycle detection in undirected graphs
- If UNION(a, b) tries to merge two nodes already in the same set,
a cycle exists.

(5) Grid / matrix problems
- Count connected islands
- Merge cells after events (flood fill but dynamic)

(6) Offline queries
- Reverse-delete algorithms
- Offline dynamic connectivity

---

3. HOW DSU WORKS (ASCII DIAGRAMS)

---

Internally, each set is a tree.
Each node stores:

* Its parent
* Optionally, its rank/size for balancing

At the beginning, each element is its own parent:

```
Element: 1  2  3  4  5
Parent:  1  2  3  4  5
```

Meaning each element is its own separate set.

---

4. ASCII EXAMPLE OF UNION OPERATIONS

---

Start: all separate
1     2     3     4     5
|     |     |     |     |
1     2     3     4     5

UNION(1, 2):

```
1
|
2     3     4     5
```

UNION(3, 4):

```
1       3
|       |
2       4       5
```

UNION(2, 3):
Here we merge the sets rooted at 1 and 3:

```
	 1
	 |
	 2
	 |
	 3
	 |
	 4       5
```

(Depending on balancing, the exact shape may differ.)

---

5. PATH COMPRESSION (WHY FIND IS FAST)

---

When we call FIND(x), DSU makes x point directly to the root of the set.

Before compression (example: find(4)):

```
1
|
2
|
3
|
4
```

After finding root(4) = 1, make 4 point directly to 1:

```
1
```

/|
2 3 4

This reduces future lookup times dramatically.

---

6. UNION BY RANK / SIZE

---

To stop trees from getting tall, we always attach the smaller tree under
the larger one.

Example:

Set A: size 5
Set B: size 2

We attach B under A:

Correct:
A-root
/
...    B-root

Wrong (would cause imbalance):
B-root
|
A-root  <-- too many nodes below

This keeps the entire structure shallow.

---

7. COMMON DSU METHODS

---

Initialize(N):
Create N sets, each element is its own parent.

Find(x):
Return the root (representative) of x’s set.
With path compression.

Union(x, y):
Merge the sets of x and y.
Uses union-by-rank/size.

SameSet(x, y):
Returns true if Find(x) == Find(y).

CountSets():
Optional: number of distinct components.

GetSize(x):
Optional: size of the set containing x.

---

8. SUMMARY

---

* DSU maintains dynamic sets of connected elements.
* UNION merges sets; FIND identifies which set an element belongs to.
* Path compression and union by rank keep operations nearly O(1).
* Essential in graph connectivity, MST (Kruskal), cycle detection,
  and dynamic grouping problems.

---

## END OF TEXT

If you want, I can also generate a "cheat-sheet style" version,
or add more ASCII for Kruskal, grids, islands, or cycle detection.
