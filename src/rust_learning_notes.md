# Rust learning notes

* Use Option<Box<dyn Trait>>, not Box<Option<dyn Trait>> which gives some unknown runtime size failure.
* floats are only partially comparable
* use `approx` crate to support equality testing of anything containing a float
* for whatever reason Rust will not accept `1` or `2` whenever a float is required. Shouldn't it be safe to cast these to the more detailed type automatically?
* need to read chapter on function pointers, closures, etc.
* need to read chapter on advanced references (Rc, etc.)
* root for RFC on delegation (including partial delegation)
* If you Box a trait object, you have to implement PartialEq yourself: https://github.com/rust-lang/rust/issues/39128. You will also need dyn-clone for Clone. Copy will not be sorry. Clone everywhere or pass references instead.
*
