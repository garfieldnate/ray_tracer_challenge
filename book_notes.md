Hi,

I fell totally in love with this book and had a ton of fun working through it. I've moved on to other projects now, and I thought I should share my final book notes before I forget!

## Performance Tips

- The greatest speed boost not mentioned in the book came from abandoning the bidirectional tree structure for groups, instead pushing down transformations on groups to each leaf node so that only one transformation matrix multiplication would occur per leaf node. This also lets you avoid the bidirectional tree structure, which can be very difficult in some languages (it was close to impossible in safe Rust!).

For example, if you load a model with thousands of triangles, then each of those individual triangles already have their own separate transformation matrices. Now if you move the model around in the scene, the entire group will have another transformation matrix. If you nest and transform groups with many primitives, you will have many more transformation matrices, and when you cast a ray many of the same matrix multiplications will be applied repeatedly.

To fix this, whenever a transformation is applied to a group, just push the transformation down to the children, who can then multiply it with their pre-existing transformation matrix to come up with a new transformation matrix. Now when you cast a ray, you only need to apply one matrix multiplication per primitive instead of one per group nesting level, and you also don't need to keep track of or loop through parent groups.

The book-keeping is a little more tricky if you want to be able to overwrite an existing group transform, though; you'll have to store the group's transformation inverse, and when a new transformation is applied you'll need to transform the children first by this inverse and then by the new transformation matrix. There is some extra care required when implementing bounding boxes, too. It saves a TON of time though, so this optimization is worth it!


- This is a smaller gain, but I'll still note it since I haven't seen it elsewhere. In the groups chapter, when making the test "Intersecting a ray with a nonempty group" pass, make sure not to sort the intersections in the intersect method of your group implementation, since this is already done in World and would be a redundant computation. You may end up sorting them in the test function itself to aid in comparison, though if it's easy in your chosen language then the conceptually simplest thing would be to store the expected and actual intersections in Set objects and then compare the sets for equality.

## Possible/Definite Errata

These are points where following the book exactly caused incorrect behavior in my implementation.

-   The "A ray misses a cube" test needs one more case: the ray is cast away from the cube. Not implementing this will make cubes always appear in front of the camera, even if placed behind it. Here is another test case where the ray should miss the cube:

    |point(0, 0, 2) | vector(0, 0, 1)|

    Fixing the book's algorithm: at the end of the method, if tmax is negative then no intersection should be reported. The algorithm given in the book allows `tmax` to be a negative number, indicating an intersection _opposite_ the ray's direction.
-   Cylinder cap intersection should NOT have `object_ray.direction.y <= REALLY_SMALL_POSITIVE_NUMBER` as a quick return.
-   Cone and cylinder side intersections required checking a.abs() and b.abs() against `REALLY_SMALL_POSITIVE_NUMBER`; exact checking against 0 did not work at all.
- For the second test case given for "Intersecting a cone with a ray", I had to change -5 to -4.999999 to get the ray to intersect with the cone.
-   The cylinder azimuth calculation should be performed with x and z, not x and y.
-   Bonus bounding box chapter: there is no bounding box specified for smooth triangles; though I did not end up implementing this, I'm certain the algorithm has to be different from that used for regular triangles.
-  Bonus UV mapping chapter: the provided cross diagram for UV mapping of a cube is wrong for up and down
    *   Up and down should both map x positively. The tests are correct. I found another chart with the correct mapping on wikipedia: https://en.wikipedia.org/wiki/Cube_mapping#/media/File:Cube_map.svg, but it also makes intuitive sense. Looking up or down from inside the cube, the observer's x never reverses from the absolute x.

## Clarifications

- In the cone section: "If a is nonzero, you’ll use the same algorithm, but with the new a, b, and c, that you used for the cylinders." I found this sentence confusing and it took a while to understand it. I think it would be more clearly written like so: "If a is nonzero, you’ll use the same algorithm (but with the new a, b, and c) that you used for the cylinders."
-   Where UV mappers use mod/%, a note should be made that there are several related operations that a programming language might represent with `%`; particularly, be careful to choose an implementation that always return positive values, even if the input is negative. See the treatment on Wikipedia here: https://en.wikipedia.org/wiki/Modulo_operation#In_programming_languages. In Rust, I needed to use the `rem_euclid` function instead of the `%` operator.
-   There isn't any note in the UV chapter on how a cylinder has to be scaled for the texture to look correct, though Jamis demonstrates it in the provided scene YAMLs. As I found a similar issue reported on StackOverflow, I ended up doing a little writeup there to explain the issue and how to fix it: https://stackoverflow.com/a/60913088/474819
-   Test "Converting a point from world to object space" should scale by 1,2,3 instead of 2,2,2 to catch more programming errors.

## Typos/Small Errata

These will only really be useful for Jamis :)

- Typo in bounding boxes chapter: inculde -> include
-   Pseudocode for CubeMap's pattern_at contains uv_cube_left, etc. but these should be cube_uv_left, etc. to be consistent with the previous declarations in the chapter
-   There are two scenarios named "Checker pattern in 2D" in the UV mapping chapter. The second one should probably be "UV mapping an image" or something like that.
-   Typo in UV chapter: "Experiment and see what you come up." (missing "with")
-   "non-cubic bounding box" is not a great name for the test in the bounding box chapter; maybe "bounding box not centered at origin" would be clearer.

