# r3bl-cmdr
<a id="markdown-r3bl-cmdr" name="r3bl-cmdr"></a>


<!-- TOC -->

- [1. What is it?](#1-what-is-it)
- [2. r3bl_rs_utils crate](#2-r3bl_rs_utils-crate)
- [3. This project is currently WIP](#3-this-project-is-currently-wip)

<!-- /TOC -->

## What is it?
<a id="markdown-what-is-it%3F" name="what-is-it%3F"></a>


You can run it using `cargo run`.

This TUI (text user interface) app showcases the use of the `r3bl_rs_utils` crate. It contains quite
a few sample apps which are meant to be relevant use cases that are relevant for developer workflows
(who are remote, and work w/ teams).

## r3bl_rs_utils crate
<a id="markdown-r3bl_rs_utils-crate" name="r3bl_rs_utils-crate"></a>


The [`r3bl_rs_utils`](https://crates.io/crates/r3bl_rs_utils) crate allows you to build fully async
TUI apps with a modern API that brings the best of reactive & unidirectional data flow architecture
from frontend web development (React, Redux, CSS, flexbox) to Rust and TUI apps. And since this is
using Tokio you get the advantages of concurrency and parallelism built-in. No more blocking on the
main thread for user input, for async middleware, or even rendering 🎉.

This framework is
[loosely coupled and strongly coherent](https://developerlife.com/2015/11/05/loosely-coupled-strongly-coherent/)
meaning that you can pick and choose whatever pieces you would like to use w/out having the
cognitive load of having to grok all the things in the codebase. Its more like a collection of
mostly independent modules that work well w/ each other, but know very little about each other.

Here are some framework highlights:

- The entire TUI framework itself supports concurrency & parallelism (user input, rendering, etc.
  are generally non blocking).
- Flexbox-like responsive layout.
- CSS-like styling.
- Redux for state management (fully async, concurrent & parallel).
- Lolcat implementation w/ a rainbow color-wheel palette.
- Support for Unicode grapheme clusters in strings.

## This project is currently WIP
<a id="markdown-this-project-is-currently-wip" name="this-project-is-currently-wip"></a>


This bin crate is being developed as a set of examples. The actual product will emerge as these
examples are evolved into features of the actual product, which is intended to be released to
developers.

> You can see the TODOs in [TODO.todo](TODO.todo).
