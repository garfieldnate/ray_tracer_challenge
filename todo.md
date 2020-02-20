# TODO

- Patterns
  - Make pattern compound, so that stripes can contain stripes, etc.
  - Additive and subtractive pattern compounds
  - Perlin perturbat
- Add `Default` trait implementation everywhere
- Switch to f64 for geometric calculations everywhere (leave f32 for colors).
  - From _Fundamentals of Computer Graphics_:
    > I suggest using doubles for geometric computation and floats for color computation. For data that occupies a lot of memory, such as triangle meshes, I suggest storing float data, but converting to double when data is accessed through member functions.
- Canvas write_pixel: fail properly for out of bounds
- cleanup after [codereview.se](https://codereview.stackexchange.com/questions/236895/color-and-canvas-implementations-in-rust-for-ray-tracer-challenge) responds
- transformation should probably all be in matrix
- Having to use & everywhere sucks for matrix multplication sucks
- BoxedPattern type can be changed to `type BoxedPattern<'a> = &'a (dyn Pattern + 'a);`. Is that better or worse?
- Profile to find any bottlenecks
- Parallelize rendering

### Ergonomics

- Switch to IntelliJ
- YAML scene file parsing

### Maybes/Ideas

- Better model of point vs. vector: typing should reflect difference
- Reorganize as [workspace](https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-section)
- reimplement everything using geometric algebra for funsies

### Compositions

- Maze on sphere with glass on top, like those toys, or like super mario galaxy
- Bowl of glass M&M's
