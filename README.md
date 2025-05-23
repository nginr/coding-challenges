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
  "sysroot_src": "${HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library",
  "crates": [
    {
      "root_module": "src/0_wc.rs",
      "edition": "2021",
      "deps": [],
      "source": {
        "include_dirs": [ ".", "./crates/*" ],
        "exclude_dirs": []
      }
    },
    {
      "root_module": "src/1_json_parser.rs",
      "edition": "2021",
      "deps": [],
      "source": {
        "include_dirs": [ ".", "./crates/*" ],
        "exclude_dirs": []
      }
    }
  ]
}
```

This is template that needs the ENV variable substituted.

### VSCode

VSCode needs a setting set for this seemingly unlinked rust file project in the `.vscode/settings.json` file.
See [Issue #15068](https://github.com/rust-lang/rust-analyzer/issues/15068)

```json
{
    "rust-analyzer.linkedProjects": [
        "rust-project.json"
    ]
}
```

### Debugging

For Debugging you may need to install the Rust-Analyzer extension and a `launch.json` file. Maybe set the `RUST_BACKTRACE=1` environment variable also.

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug Rust",
            "type": "cppdbg",
            "request": "launch",
            "program": "${workspaceFolder}/build/1_rjsonp",
            "args": ["${workspaceFolder}/tests/1_json_parser/step2/invalid.json"],
            "cwd": "${workspaceFolder}",
            "stopAtEntry": true,
            "MIMode": "gdb",
            "setupCommands": [
                {
                    "description": "Enable pretty-printing for gdb",
                    "text": "-enable-pretty-printing",
                    "ignoreFailures": true
                },
                {
                    "description": "Set Disassembly Flavor to Intel",
                    "text": "-gdb-set disassembly-flavor intel",
                    "ignoreFailures": true
                }
            ]
        }
    ]
}
```
