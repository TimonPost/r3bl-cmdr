# r3bl-cmdr
<a id="markdown-r3bl-cmdr" name="r3bl-cmdr"></a>


<!-- TOC -->

- [1. This project is WIP](#1-this-project-is-wip)
- [2. The plan is to ...](#2-the-plan-is-to-)
- [3. The following has already been done](#3-the-following-has-already-been-done)

<!-- /TOC -->

## This project is WIP
<a id="markdown-this-project-is-wip" name="this-project-is-wip"></a>


You can run it using `cargo run`.

This TUI (text user interface) app showcases the use of the `r3bl_rs_utils` crate. It contains quite
a few sample apps which are meant to be relevant use cases that are relevant for developer workflows
(who are remote, and work w/ teams).

The [`r3bl_rs_utils`](https://crates.io/crates/r3bl_rs_utils) crate allows you to build fully async
(parallel and concurrent via Tokio) TUI apps with a modern API that integrates the best of frontend
web development. Here are some framework highlights:

- The entire TUI framework itself supports concurrency & parallelism (user input, rendering, etc.
  are generally non blocking).
- You can use:
  - something like Flexbox for responsive layout.
  - something like CSS for styling.
  - Redux for state management (fully async, concurrent & parallel).
  - A lolcat implementation w/ a rainbow color-wheel palette.

## The plan is to ...
<a id="markdown-the-plan-is-to-..." name="the-plan-is-to-..."></a>


- Move the `lib` portion to `r3bl_rs_utils` crate (the `tui` part).
- The `bin` portion stays in this repo, and graduates from examples into the actual TUI app which is
  released a product for developers.

> You can see the TODOs in [TODO.todo](TODO.todo).

## The following has already been done
<a id="markdown-the-following-has-already-been-done" name="the-following-has-already-been-done"></a>


- Move some of the `lib` portion into `r3bl_rs_utils` crate
  [the `tui-core` part](https://github.com/r3bl-org/r3bl-rs-utils#tui-experimental).

> You can see the DONEs in [DONE.todo](DONE.todo).
