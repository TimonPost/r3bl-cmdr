# r3bl-cmdr

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [This project is WIP](#this-project-is-wip)
- [The plan is to](#the-plan-is-to)
- [The following has already been done](#the-following-has-already-been-done)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# This project is WIP

You can run it using `cargo run` and it shows a simple TUI app w/ 2 column layout that
uses something like Flexbox.

- It uses Redux for state management.
- And it uses Tokio for concurrency and parallelism.
- The entire TUI framework itself supports concurrency & parallelism (user input,
  rendering, etc).
- It has a lolcat implementation w/ a rainbow color-wheel palette.

> You can see the TODOs in [TODO.todo](TODO.todo).

# The plan is to

- Move the `lib` portion to `r3bl_rs_utils` crate (the `tui` part).
- The `bin` portion stays in this repo, and becomes the actual TUI app.

# The following has already been done

- Move some of the `lib` portion into `r3bl_rs_utils` crate
  [the `tui-core` part](https://github.com/r3bl-org/r3bl-rs-utils#tui-experimental).

> You can see the DONEs in [DONE.todo](DONE.todo).
