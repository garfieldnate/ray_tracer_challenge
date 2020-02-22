# TODO

- reduce mutability everywhere possible, including tests, using scoped mutability and also ..Default::default() syntax
- Patterns
  - Make pattern compound, so that stripes can contain stripes, etc.
  - Additive and subtractive pattern compounds
  - Perlin perturbation - material belongs in pattern; just make default pattern a solid color
- Switch to f64 for geometric calculations everywhere (leave f32 for colors).
  - From _Fundamentals of Computer Graphics_:
    > I suggest using doubles for geometric computation and floats for color computation. For data that occupies a lot of memory, such as triangle meshes, I suggest storing float data, but converting to double when data is accessed through member functions.
- Canvas write_pixel: fail properly for out of bounds
- cleanup after [codereview.se](https://codereview.stackexchange.com/questions/236895/color-and-canvas-implementations-in-rust-for-ray-tracer-challenge) responds
- transformation should probably all be in matrix
- Having to use & everywhere for matrix, tuple or color multplication sucks
- Integrate error-chain if needed (http://brson.github.io/2016/11/30/starting-with-error-chain)

### Performance

- Profile to find any bottlenecks
- Parallelize rendering
- (also ergonomics) new canvas implementation that displays during render
- switch to matrix library (open BLAS or whatever)
- can we use a GPU somehow?

### Ergonomics

- YAML scene file parsing
- Color constructor that takes hex string
- display a grid
- Switch to IntelliJ

### Maybes/Ideas

- Better model of point vs. vector: typing should reflect difference
- Reorganize as [workspace](https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-section)
- reimplement everything using geometric algebra for funsies (https://crypto.stanford.edu/~blynn/haskell/ga.html)
- lines
- electricity

### Compositions

- Maze on sphere with glass on top, like those toys, or like super mario galaxy
- Bowl of glass M&M's
