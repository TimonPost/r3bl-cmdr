<p align="center">
  <img src="r3bl-term.svg" height="128px">
</p>

# Context

<code><font color="#FD2F53">R</font><font color="#FC2C57">3</font><font color="#FB295B">B</font><font color="#FA265F">L</font><font color="#F92363">
</font><font color="#F82067">T</font><font color="#F61D6B">U</font><font color="#F51A6F">I</font><font color="#F31874">
</font><font color="#F11678">l</font><font color="#EF137C">i</font><font color="#ED1180">b</font><font color="#EB0F84">r</font><font color="#E90D89">a</font><font color="#E60B8D">r</font><font color="#E40A91">y</font><font color="#E10895">
</font><font color="#DE0799">&amp;</font><font color="#DB069E">
</font><font color="#D804A2">s</font><font color="#D503A6">u</font><font color="#D203AA">i</font><font color="#CF02AE">t</font><font color="#CB01B2">e</font><font color="#C801B6">
</font><font color="#C501B9">o</font><font color="#C101BD">f</font><font color="#BD01C1">
</font><font color="#BA01C4">a</font><font color="#B601C8">p</font><font color="#B201CB">p</font><font color="#AE02CF">s</font><font color="#AA03D2">
</font><font color="#A603D5">f</font><font color="#A204D8">o</font><font color="#9E06DB">c</font><font color="#9A07DE">u</font><font color="#9608E1">s</font><font color="#910AE3">e</font><font color="#8D0BE6">d</font><font color="#890DE8">
</font><font color="#850FEB">o</font><font color="#8111ED">n</font><font color="#7C13EF">
</font><font color="#7815F1">d</font><font color="#7418F3">e</font><font color="#701AF5">v</font><font color="#6B1DF6">e</font><font color="#6720F8">l</font><font color="#6322F9">o</font><font color="#5F25FA">p</font><font color="#5B28FB">e</font><font color="#572CFC">r</font><font color="#532FFD">
</font><font color="#4F32FD">p</font><font color="#4B36FE">r</font><font color="#4739FE">o</font><font color="#443DFE">d</font><font color="#4040FE">u</font><font color="#3C44FE">c</font><font color="#3948FE">t</font><font color="#354CFE">i</font><font color="#324FFD">v</font><font color="#2E53FD">i</font><font color="#2B57FC">t</font><font color="#285BFB">y</font></code>

We are working on building command line apps in Rust which have rich text user interfaces (TUI). We
want to lean into the terminal as a place of productivity, and build all kinds of awesome apps for
it.

1. 🔮 Instead of just building one app, we are building a library to enable any kind of rich TUI
   development w/ a twist: taking concepts that work really well for the frontend mobile and web
   development world and re-imagining them for TUI & Rust.

   - Taking things like React, JSX, CSS, and Redux, but making everything async (they can be run in
     parallel & concurrent via Tokio).
   - Even the thread running the main event loop doesn't block since it is async.
   - Using proc macros to create DSLs to implement CSS & JSX.

2. 🌎 We are building apps to enhance developer productivity & workflows.

   - The idea here is not to rebuild tmux in Rust (separate processes mux'd onto a single terminal
     window). Rather it is to build a set of integrated "apps" (or "tasks") that run in the same
     process that renders to one terminal window.
   - Inside of this terminal window, we can implement things like "app" switching, routing, tiling
     layout, stacking layout, etc. so that we can manage a lot of TUI apps (which are tightly
     integrated) that are running in the same process, in the same window. So you can imagine that
     all these "app"s have shared application state (that is in a Redux store). Each "app" may also
     have its own Redux store.
   - Here are some examples of the types of "app"s we want to build:
     1. multi user text editors w/ syntax highlighting
     2. integrations w/ github issues
     3. integrations w/ calendar, email, contacts APIs

# About this binary crate: r3bl-cmdr

`r3bl-cmdr` is the second thing that's described above.

You can run it using `cargo run`.

This TUI (text user interface) app showcases the use of the `r3bl_rs_utils` crate. It contains quite
a few sample apps which are meant to be relevant use cases that are relevant for developer workflows
(who are remote, and work w/ teams).

# Contributing

<a id="markdown-contributing" name="contributing"></a>

This binary crate is being developed as a set of examples. The actual product will emerge as these
examples are evolved into features of the actual product, which is intended to be released to
developers.

Please read our [community contributing guidelines here](./CONTRIBUTING.md).
