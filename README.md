# Pink Cute Rust (basic Quest Rust mod template)

This mod is fairly barebones and intends to show almost everything you'd need to get a basic Rust mod for Quest. 

While it is built for Beat Saber, it does not exclusively target it.

# Requirements:
- Cargo
- QPM
- [Bear](https://github.com/rizsotto/Bear) (optional, generating compile commmands)
- clangd (Optional, C++ linting)


# Caveats:
Since Rust is heavily oriented towards static linking rather than the C++ "use what you include" philosophy, it links the large generated `bs-cordl` crate in its entirety PER build, regardless of cache. Unfortunately, this means a compile time of 2 minutes even with the compile time friendly `dev` profile. 

As for solving this, it is likely [`wild`](https://github.com/davidlattimore/wild) would solve this with its endgoal of being an incremental linker. 🤞

Additionally, linting with either `cargo check` and `cargo clippy` takes a good amount of time since it tries to read the entire `bs-cordl` tree.

Help would graciously be appreciated to resolving these issues

# Promises
- Memory safe Rust mods
- Much better error handling
- High quality libraries for networking, math, web etc.
- Testing frameworks
- Cross platform mods with Unity desktop platforms e.g Windows, Linux and Mac x86/arm64
- Asynchronous code!
- Support for multiple Unity versions without prebuilt configurations.

# Things to note


- There is included CI workflow for building QMod. Releases or QPM publishes not included.

- QPM config is included for the purposes of using `cordl` in your C++ files. However, you can also use QPM for interfacing with existing C++ mods such as SongCore with C++ and `cc`. You must configure your build process in `build.rs` to support this, as CMake is not part of the build process.
- `qpm s build` handles building the qmod with NDK as needed. Note, builds as release.
- `qpm qmod zip` Prepares the mod for distribution. Builds as release.
- `qpm s copy` copies the `.so` to your quest. Builds as debug.

- Classes are locked behind feature flags in `Cargo.toml` to reduce dependency tree size. However, this is quite large per class as each class recursively enables its own dependencies.

- This does not depend on any C++ code except for `inline-hook`. 

- FFI with existing mods is not a main goal. For this, take advantage of the `cc` crate's ability to combine C++ and Rust.

- `quest-hook-rs`, the library managing most of the IL2CPP work, is quite unfinished.
- `cordl` depends on `quest-hook-rs`'s feature capacity. Thus, these features are not implemented as of 7/20/2025.
https://discord.com/channels/994470435100033074/1011387336245911682/1315148085801914369
    - Ptr<T> type
    - ByRef<T> type
    - ByRefMut<T> type
    - constant enum type fields on a non-enum type
    - value type boxing
    - value type conversion to interfaces
    - generic methods (?)
    - static field getters/setters
    - Feature parity with `bs-hooks`.