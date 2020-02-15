# TODO

* Switch to f64 for geometric calculations everywhere (leave f32 for colors).
    - From _Fundamentals of Computer Graphics_:
    > I suggest using doubles for geometric computation and floats for color computation. For data that occupies a lot of memory, such as triangle meshes, I suggest storing float data, but converting to double when data is accessed through member functions.
* Move many datatypes to their own files. Maybe a new directory for shapes.
* Canvas write_pixel: fail properly for out of bounds
* cleanup after [codereview.se](https://codereview.stackexchange.com/questions/236895/color-and-canvas-implementations-in-rust-for-ray-tracer-challenge) responds
* transformation should probably all be in matrix
* Having to use & everywhere sucks for matrix multplication sucks
* Projectile seems surprisingly slow. Are we copying data when we shouldn't?
* When you import the point!() or vector!() macros, you have to also manually import build_tuple and Tuple. Can that be automated?


### Maybes/Ideas

* Better model of point vs. vector: typing should reflect difference
* Reorganize as [workspace](https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-section)
* reimplement everything using geometric algebra for funsies


### Rust Wishes
* I wish that VSCode could auto-format macros. Doesn't seem to do indenting automatically.
* I wish that the imports were auto-organized to differentiate between macros and other stuff.
