# Rust learning notes

* Use Option<Box<dyn Trait>>, not Box<Option<dyn Trait>> which gives some unknown runtime size failure.
* floats are only partially comparable
* use `approx` crate to support equality testing of anything containing a float
* for whatever reason Rust will not accept `1` or `2` whenever a float is required. Shouldn't it be safe to cast these to the more detailed type automatically?
* need to read chapter on function pointers, closures, etc.
* need to read chapter on advanced references (Rc, etc.)
* root for RFC on delegation (including partial delegation)
* root for RFC on default parameter values
* If you Box a trait object, you have to implement PartialEq yourself: https://github.com/rust-lang/rust/issues/39128. You will also need dyn-clone for Clone. Copy will not be sorry. Clone everywhere or pass references instead.
* Always great to auto-derive these when possible: Clone, Copy, Debug, Eq/PartialEq, Default
    - Eq will get reduced to PartialEq if you have any floats
    - Copy will stop being available with any pointers (Box, etc.)
* When should I use a plain &dyn Thing and when should I use a Box<dyn Thing>

### VS Code Wishes
* I wish that VSCode could auto-format macros. Doesn't seem to do indenting automatically.
* I wish that the imports were auto-organized to differentiate between macros and other stuff.

## VS Code Problems
* Using MyStruct::new() in a file where it is imported, VS code cannot suggest what to import
* Oh my gosh waiting for full recompilation everytime I change something, between every accepted suggestion, is awful! No suggestions work while compiling.
* Sometimes have to restart to properly analyze a file. Particularly, when creating a new file in the bin directory, it won't be analyzed until restarting VS Code.
*
