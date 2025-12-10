# Why Part 2 Works

Part 2 restricts the rectangle to contain **only red or green tiles**.

Green tiles are defined implicitly by the shape formed when all red
tiles are connected in order, wrapping around to form a closed
polygon.

This polygon divides the plane into:

* **Inside** → green
* **Outside** → never allowed

Because of this restriction, only rectangles **completely inside the
polygon** are valid.

To efficiently test this for every pair of red tiles, the algorithm
uses several key ideas:

---

## 1. Build the polygon edges

The input is a sequence of red points.

Each consecutive pair forms a straight edge, and the last point
connects back to the first.

This gives us every segment that forms the boundary of the green
region.

---

## 2. Generate all possible rectangle candidates

Each rectangle is defined by choosing:

* one red tile as the top-left / bottom-left corner
* one red tile as the opposite corner

This creates all axis-aligned rectangles between every pair of red
points.

We compute:

```
minX = min(a.x, b.x)
maxX = max(a.x, b.x)
minY = min(a.y, b.y)
maxY = max(a.y, b.y)
area = (maxX - minX + 1) * (maxY - minY + 1)
```

The rectangle is **only valid** if it lies entirely inside the
polygon.

---

## 3. Manhattan-distance pruning (performance optimization)

Before checking collisions, we use:

```
if (Manhattan(a, b))^2 <= result:
	skip
```

Why?

* The Manhattan distance gives a *lower bound* on how much a rectangle
  *could* grow.

* If even the maximum possible expansion cannot beat the current best
  `result`, then there's no need to test this pair.

This cuts down the number of expensive intersection tests.

---

## 4. Rectangle–Edge overlap test

To verify that the rectangle stays inside the green area, we check
**whether any polygon edge intersects the rectangle**.

The check is intentionally simple:

```
edge overlaps rectangle bounding box?
```

If **any** edge intersects the rectangle, then the rectangle crosses
the boundary and is invalid.

This replaces full point-in-polygon and full line–rectangle
intersection.  It works because:

* The polygon has no diagonal edges
* All edges are axis-aligned
* Any edge entering the rectangle implies the boundary passes through
  it

This matches exactly the original problem constraints.

---

## 5. Keep the largest valid rectangle

For any rectangle that passes:

* Manhattan pruning
* Edge intersection check

We compare its area to the current best value and update it if larger.

---

# Summary (TL;DR)

Part 2 works because:

1. **The red tile list forms a polygon**.
2. **Green tiles are the inside of that polygon**.
3. **A valid rectangle must lie entirely inside** this polygon.
4. For every pair of red tiles, we:

   * Build its bounding rectangle
   * Prune impossible candidates using Manhattan distance
   * Check if any polygon edge crosses that rectangle
5. If no edge crosses, the rectangle is valid.
6. Track the maximum area found.

This reproduces the exact logic needed:

choose the largest rectangle whose interior does not cross the polygon
boundary.
