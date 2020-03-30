# Rust learning notes

-   Use `Option<Box<dyn Trait>>`, not `Box<Option<dyn Trait>>` which gives some unknown runtime size failure.
-   floats are only partially comparable. This is super annoying! There's no min/max function for floats, for example. If the Rust designers care about catching this type of error, then they should have also provided a floating point type that is gauranteed not to be NaN or infinity. Of course, they apparently know this, and redoing numbers has been on their todo for a while. float-ord crate sort of solves it.
-   use `approx` crate to support equality testing of anything containing a float
-   for whatever reason Rust will not accept `1` or `2` whenever a float is required. Shouldn't it be safe to cast these to the more detailed type automatically?
-   root for RFC on delegation (including partial delegation)
-   root for RFC on default parameter values
-   If you Box a trait object, you have to implement PartialEq yourself: https://github.com/rust-lang/rust/issues/39128. You will also need dyn-clone for Clone. Copy will not be sorry. Clone everywhere or pass references instead.
-   Always great to auto-derive these when possible: Clone, Copy, Debug, Eq/PartialEq, Default
    -   Eq will get reduced to PartialEq if you have any floats
    -   Copy will stop being available with any pointers (Box, etc.)
-   Use a plain &dyn Thing when the lifetime parameter requirement won't mess up the whole codebase and ownership is totally clear. Otherwise use Box<dyn Thing>.
-   Rust is very careful to distinguish between mutable and immutable variables, but you can rebind immutable variables for some reason. I guess it's memory safe, but it seems like a possible footgun if you expect better immutability gaurantees. I'm used to TypeScript, where you just declare locals as consts all the time.
-   I wish assert_abs_diff_eq from the approx crate would take an additional message string with format args, like the built-in assert! and friends do. See https://github.com/brendanzab/approx/issues/44.
    -   Would also be great if the AproxEq trait were implemented for tuples.

*   Linting with clippy:
    -   Currently you can't enable/disable lints, including for clippy, in a config file. Issue: https://github.com/rust-lang/cargo/issues/5034. I can't really fix all of the direct float comparison spots, and sometimes it's for sure safe.
    -   Clippy says that `0.121_218_32` has a mistyped literal suffix and suggests changing the end to `f32` That's clearly a bug. The actual number is supposed to end in `32`.
    -   I think for now Clippy has to be a "run once in a while for improvements" tool, not a "run all the time to catch mistakes" tool.

-   Lack of visibility controls and relative imports is kind of a bummer. I expected this part to be a lot more modern.
-   I really, _really_ wish I could do `println!(3)` instead of `println!("{}", 3)`.
-   Interesting, in Java I would always return a vague type, but in Rust if you return a trait type then you have to box it. Is that inefficient? I don't think it can be completely 0 cost, since the reason the box is required there is that the plain trait object can't be passed back normally via the stack because the size is unknown. It has to be allocated and a pointer returned instead.
-   Just realized I've been glossing over generics and using dynamics traits everywhere when I really shouldn't. Generics allow the compiler to specialize our code, like C++ templates! Trait objects, on the other hand, cause dynamic dispatch, which is more expensive. http://blog.metrink.com/blog/2018/03/30/rust-trait-objects-vs-generics/
-   cargo-flamegraph is freaking AWESOME! I never found it this easy to profile code before! One problem: on Mac you have to run it with sudo because of permissions that dtrace needs. This can also lead to the build directory being owned by a superuser, so you have to chown it back after profiling.
-   Man I wish contain-rs were maintained! That's a seriously important project. LinkedHashMap, etc.
-   Need better testing method for RNG-seeded operations;rand::rngs::mock::StepRng::new(0, step) is fine if using gen<i32> or something; but get<f32> gives an exact spread over all possible f32 values, which are not conducive to manual testing. I'd rather be able to write `float_rng(0., 0.1)` and have it generate .1, , .2, etc. Even better would be a test RNG that takes a sequence from the user.
-   setting "rust.build_lib": true does nothing in vscode. The documentation says both that it is an unstable feature and that no features require "rust.unstable_features" to be enabled, but I also tried "rust.unstable_features" enabled and vscode told me I had to use rust nightly for that.
-   To run the flamegraph binary with an argument for the executable being perfed:
    \$ sudo flamegraph "target/release/here_be_dragons /Users/nathanglenn/Downloads/dragon.obj"
    Also don't forget to put this in your Cargo.toml:

        [profile.release]
        debug = true

-   If you map from Objects to Box<dyn Trait> objects, you'll probably need to put an `as _` on the box creation step to get the compiler to understand the types correctly. I'm not sure why this is required. Something about the compiler needing permission to do a coercion? Maybe it's just one spot that hasn't been ergonomized yet. It seems like `Box<T: Shape>` should always implement `Into<Box<dyn Shape>>` See https://users.rust-lang.org/t/cannot-extend-a-vec-of-trait-objects/28129.
-   Would be helpful if I could use `assert_abs_diff_eq!` on tuples automatically
-   There's no enum map :/ There is an enum-map crate, but you can only use basic types for values. I tried to use a box type, and the compiler complained about not being able to move values.
- Cool feature, but important to know about and design is maybe a little questionable. If you create an iterator and call `enumerate` on it, you'll get an iterator over `(usize, value)`. If you call `next` to get a single value from it, and then call `enumerate` after that, you'll instead get `(usize, (usize, value))`, which gives you the index into the current iteration and also the overall iterator index, respectively. I like that this information is provided, but I don't understand how the type inference worked out on it, and it was a surprising type error gotcha for me.
- typed-builder is really amazing, but coming from the Java world and Lombok I'm still missing two important features:
    * Override any piece of the setters or constructor
    * Turn a resulting object back into a builder

### VS Code Wishes

-   I wish that VSCode could auto-format macros. Doesn't seem to do indenting automatically.
-   I wish that the imports were auto-organized to differentiate between macros and other stuff.
-   A "fix all problems like this in file" would be fantastic, especially unambiguous for imports (always choose std over core unless core is already being used).
-   Replace in selection is impossible with multiple selections
- Cannot paste multiple lines into multiple selections. Each line just gets placed in a different selection

## VS Code Problems

-   Using MyStruct::new() in a file where it is imported, VS code cannot suggest what to import
-   Oh my gosh waiting for full recompilation everytime I change something, between every accepted suggestion, is awful! No suggestions work while compiling.
-   Sometimes have to restart to properly analyze a file. Particularly, when creating a new file in the bin directory, it won't be analyzed until restarting VS Code.

## Preview Bug

Sometimes Preview copies the wrong image to the clipboard. Usually happens when I'm generating the same image multiple times with the same name. The display will be correct, but when I go to copy, I still get the old image.
