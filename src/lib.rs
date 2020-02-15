// Following the book, we'll make lots of functions for use later. Everything is going to be dead code for a while.
#![allow(dead_code)]

#[cfg_attr(test, macro_use)]
extern crate approx;

// It's important that mods with macros come first so that they are available in other mods.
// The macros will automatically be available in all following mods. However, for binaries in
// the bin directory, the macros must be imported from the root crate, like
// `use ray_tracer_challeng::point`, etc.
// The client must also always import `build_tuple` and `Tuple`, etc. for the macro usage to compile.
#[macro_use]
pub mod matrix;
#[macro_use]
pub mod tuple;
#[macro_use]
pub mod color;

pub mod camera;
pub mod canvas;
pub mod light;
pub mod material;
pub mod ray;
pub mod shape;
pub mod transformations;
pub mod world;
