# TODO

* Canvas write_pixel: fail properly for out of bounds
* cleanup after [codereview.se](https://codereview.stackexchange.com/questions/236895/color-and-canvas-implementations-in-rust-for-ray-tracer-challenge) responds
* matrix declaration totally sucks. Can we make a macro or something?
* Tuple also sucks. I'm sick of typing .0 all of the time.
* transformation should probably all be in matrix
* Having to use & everywhere sucks for matrix multplication sucks
* Projectile seems surprisingly slow. Are we copying data when we shouldn't?


### Maybes/Ideas

* Better model of point vs. vector: typing should reflect difference
* Reorganize as [workspace](https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-section)
* reimplement everything using geometric algebra for funsies
