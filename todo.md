Up next: anti-aliasing (FXAA: https://catlikecoding.com/unity/tutorials/advanced-rendering/fxaa/)

# TODO

-   Why do our glass CSG balls with subtracted cylinders look so wrong?
-   reduce mutability everywhere possible, including tests, using scoped mutability and also ..Default::default() syntax
-   Patterns
    -   Make pattern compound, so that stripes can contain stripes, etc.
    -   Additive and subtractive pattern compounds
    -   Perlin perturbation - material belongs in pattern; just make default pattern a solid color
-   Switch to f64 for geometric calculations everywhere (leave f32 for colors).
    -   From _Fundamentals of Computer Graphics_:
        > I suggest using doubles for geometric computation and floats for color computation. For data that occupies a lot of memory, such as triangle meshes, I suggest storing float data, but converting to double when data is accessed through member functions.
-   do realistic shadow effect for transparent things
    -   curently objects can opt out of having a shadow, but that's not really realistic
    -   keyword in graphics is "caustic"
-   Canvas write_pixel: fail properly for out of bounds
-   transformation should probably all be in matrix
-   Integrate error-chain if needed (http://brson.github.io/2016/11/30/starting-with-error-chain)
-   Support multiple lights

### Performance

-   auto-decide when a refract/reflect recursion should end. The reflect_refract binary is 10x faster at 5 instead of 20 but I don't see a visual difference. Seems like maybe we could check if something is close to 0.
-   (also ergonomics) see if we can replace trait objects with generics everywhere.
-   Triangle meshes as mentioned in the book, instead of simple groups
    -   currently if we set the material on a group containing an OBJ, it takes a lot of memory and time to set it on each child shape.
-   Parallelize rendering
-   (also ergonomics) new canvas implementation that displays during render
-   switch to matrix library (open BLAS or whatever)
-   can we use a GPU somehow?

### Ergonomics

-   Should somehow require that a shape be made immutable ("lock") before allowing bounding boxes to be calculated, since GroupShape and CSG cache them
-   use `derivative` crate's functionality in more places
-   Parse more details of OBJ files, or at least ignore everything after / in polygon lines
-   String ID's for all shapes. Would make testing way easier
-   YAML scene file parsing
-   display a grid
-   Switch to IntelliJ
-   Convenience function for creating skyboxes
-   Switch to just a pattern value in material, no color; typed-builder and derive-builder both have difficulties with this (some compile error about not being able to copy a boxed value)

### Maybes/Ideas

-   Twist transformation
-   Better model of point vs. vector: typing should reflect difference
-   reimplement everything using Grassman algebra for funsies
-   lines
-   electricity
-   pattern that looks like damascus steel

### Compositions

-   Maze on sphere with glass on top, like those toys, or like super mario galaxy
-   Bowl of glass M&M's

Notes from book about reflection/refraction:

-   When rendering glass or any similar material, set both transparency and reflectivity to high values, 0.9 or even 1. This allows the Fresnel effect to kick in, and gives your material an added touch of realism!
-   Because the reflected and refracted colors are added to the surface color, they’ll tend to make such objects brighter. You can tone down the material’s diffuse and ambient properties to compensate. The more transparent or reflective the surface, the smaller the diffuse property should be. This way, more of the color comes from the secondary rays, and less from the object’s surface.
-   If you’d like a subtly colored mirror, or slightly tinted glass, use a very dark color, instead of a very light one. Red glass, for instance, should use a very dark red, almost black, instead of a very bright red. In general, the more reflective or transparent the surface, the darker its surface color should be. Note that if you add color, make sure that you have some diffuse and possibly ambient contribution, too; otherwise, your surface will render as black regardless of what color you give to it.
-   Reflective and transparent surfaces pair nicely with tight specular highlights. Set specular to 1 and bump shininess to 300 or more to get a highlight that really shines.
