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
