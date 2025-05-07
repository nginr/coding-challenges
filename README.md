# Coding Challenges in Crust.

Implementations of challenges from [Coding Challenges](https://codingchallenges.fyi/challenges/intro) in [Crust](https://github.com/tsoding/crust).
A bare bone Rust that is more like C. Using Rust's ffi interface to C, to use C libraries and libc.
Inspired by [B](https://github.com/tsoding/b) by [Tsoding](https://www.twitch.tv/tsoding) (Mr. Strimer).

## Dependencies
- [nob.h](https://github.com/tsoding/nob.h)
- [flag.h](https://github.com/tsoding/flag.h)

Single header-file, stb-style libraries.

## Build Process

No `Cargo`, just `rustc`, `gcc` and `make`.

```sh
$ git clone https://github.com/nginr/coding-challenges
$ cd coding-challenges
$ make
```

Artifacts are in `build/`. They can be executed by:

```sh
$ ./build/XX [Options] (FILE)
```

## Rust-Analyzer / LSP 

Since this project does not use Cargo, `rust-analyzer` needs speacial instructions to provide LSP functionalities
to IDEs or alike.

### Config

For non-cargo projects, Rust-Analyzer needs a project level `rust-project.json` file to describe where each file is.
See [Non-Cargo projects](https://rust-analyzer.github.io/book/non_cargo_based_projects.html).

```json
{
  "sysroot": "${HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu",
  "sysroot_src": "${HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library",
  "crates": [
    {
      "root_module": "0_wc-rs/src/main.rs",
      "edition": "2021",
      "source": {
        "include_dirs": ["0_wc-rs/src", "crates/"],
        "exclude_dirs": []
      }
    },
    {
      "root_module": "1_json_parser-rs/src/main.rs",
      "edition": "2021",
      "source": {
        "include_dirs": ["1_json_parser-rs/src", "crates/"],
        "exclude_dirs": []
      }
    }
  ]
}
```

This is template that needs the ENV variable substituted.

### VSCode

VSCode needs a setting set for this seemingly unlinked rust file project in the `.vscode/settings.jaon` file.
See [Issue #15068](https://github.com/rust-lang/rust-analyzer/issues/15068)

```json
{
    "rust-analyzer.linkedProjects": [
        "rust-project.json"
    ]
}
```

