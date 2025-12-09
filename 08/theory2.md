---

	  D S U / U N I O N - F I N D   C H E A T  S H E E T

---

## WHAT IT IS

A structure that keeps elements grouped into disjoint sets.
Two operations:

* FIND(x): returns the representative (root) of x’s set.
* UNION(x, y): merges the sets of x and y.

## WHY IT’S GOOD

Almost O(1) per operation (inverse Ackermann).
Perfect for dynamic connectivity problems.

## WHAT IT SOLVES

* Check if two nodes are connected
* Kruskal’s Minimum Spanning Tree
* Detect cycles in undirected graphs
* Count connected components
* Manage islands (grid problems)
* Offline dynamic connectivity
* Grouping users / friends / networks

---

## BASIC INTERNAL IDEA (FOREST OF TREES)

Each set = one tree.
Each node has a parent pointer.

Initially (N = 5):
1  2  3  4  5
|  |  |  |  |
1  2  3  4  5

UNION(1,2):
1
|
2   3   4   5

UNION(3,4):
1     3
|     |
2     4     5

UNION(2,3):
1
|
2
|
3
|
4       5

---

## PATH COMPRESSION (MAKE FIND FAST)

Before FIND(4):
1
|
2
|
3
|
4

After FIND(4):
 1
/|
2 3 4

This flattens the structure → faster future operations.

---

## UNION BY RANK / SIZE (KEEP TREES SHALLOW)

Always attach the smaller tree under the larger one.

Correct:
big-root
/
...       small-root

Incorrect (slows everything):
small-root
|
big-root

Combined with path compression → practically constant time.

---

## METHODS (THE CORE API)

Initialize(N):
parent[i] = i
size[i] = 1  (or rank[i] = 0)

Find(x):
if parent[x] != x:
parent[x] = Find(parent[x])   (path compression)
return parent[x]

Union(x, y):
rx = Find(x)
ry = Find(y)
if rx == ry: return
attach smaller to larger
if size[rx] < size[ry]: swap(rx, ry)
parent[ry] = rx
size[rx] += size[ry]

SameSet(x, y):
return Find(x) == Find(y)

---

## COMMON PATTERNS

CYCLE DETECTION (undirected graph):
for each edge (u, v):
if SameSet(u, v): cycle!
else: Union(u, v)

KRUSKAL MST:
sort edges by weight
for each edge (u, v):
if !SameSet(u, v):
Union(u, v)
add edge to MST

GRID TO DSU (convert (row, col) → id):
id = row * cols + col
Union cells that are adjacent and compatible
Find(id) to check connectivity

COUNT COMPONENTS:
count roots: number of i where parent[i] == i

---

## ASCII CONNECTIVITY CHECK

Given:
A---B     C---D
\   /
E

The DSU roots evolve:
Union(A,B)
Union(C,D)
Union(B,E)
Union(D,E)

Eventually:
ROOT
/   |
A    B    E
/
C   D

So SameSet(A, C) = true.

---

## KEY TAKEAWAYS

* DSU == efficient merging + checking membership
* Path compression + union-by-size == speed
* Core tool in graph algorithms
* Very simple but extremely powerful

---
