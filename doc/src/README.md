# Pen

The programming language for scalable software development

## Vision

Pen is designed for software development by a large number of people and/or over a long time.

To make such development efficient, it focuses on:

- Maintainability
  - Everyone can learn the language and participate in actual development quickly.
  - Developers can focus on application logic rather than ever-changing implementation details.
- Portability
  - Programs written in the language can be ported to different platforms.

## Features

### Minimal design

- Its syntax and type system are simple and easy to learn.
- Its minimal language features keep codes consistent.

### System injection

- System APIs are always injected into entry points of applications.
- That isolates and protects application logic from implementation details bringing software's long expectancy as well as maintainability.
- Developers can define their own system APIs and build applications on top of them.

### Even more...

#### Static typing

Data types are checked at compile time so that developers can catch errors earlier.

#### Immutable values

All values are immutable, which leads to predictable and testable codes.

#### Pure functions by default

Functions are pure; they work just like math functions unless developers inject side effects explicitly.

#### Errors as values

Errors are merely data. Its special syntax provides a convenient way to handle errors inside each function.

#### Cross compile

The compiler and runtime support different CPU architectures, operating systems, web browsers and [WASI](https://wasi.dev/) (WIP.)

#### Foreign Function Interface (FFI)

Its C/[Rust](https://www.rust-lang.org/) FFI provides interoperability with other languages.

#### Deterministic tests (WIP)

Unit tests are deterministic and realize reliable continuous integration.

#### Asynchronous operation (WIP)

Every function is possibly asynchronous while called in the same way as synchronous ones.

#### Parallel computation (WIP)

The runtime and library provide tools for thread-safe parallel computation that leverage multi-core CPUs.

## License

Dual-licensed under [MIT](https://github.com/pen-lang/pen/blob/main/LICENSE-MIT) and [Apache 2.0](https://github.com/pen-lang/pen/blob/main/LICENSE-APACHE).