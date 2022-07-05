/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

//! Background information on terminals, VT100, ANSI, ASCII, etc.
//!
//! - crossterm docs: https://docs.rs/crossterm/latest/crossterm/index.html
//!   - Raw mode: https://docs.rs/crossterm/0.23.2/crossterm/terminal/index.html#raw-mode
//!   - Event Poll vs block: https://github.com/crossterm-rs/crossterm/wiki/Upgrade-from-0.13-to-0.14#115-event-polling
//!   - Async event read eg: https://github.com/crossterm-rs/crossterm/blob/master/examples/event-stream-tokio.rs
//!   - Async event read src: https://github.com/crossterm-rs/crossterm/blob/master/src/event/stream.rs#L23
//! - Tutorial: https://medium.com/@otukof/build-your-text-editor-with-rust-part-2-74e03daef237
//! - Raw mode: https://en.wikipedia.org/wiki/POSIX_terminal_interface#Non-canonical_mode_processing
//! - Canonical mode: https://en.wikipedia.org/wiki/POSIX_terminal_interface#Canonical_mode_processing
//! - Control characters & escape codes:
//!   - ANSI escape codes: https://en.wikipedia.org/wiki/ANSI_escape_code
//!     - Windows support: https://en.wikipedia.org/wiki/ANSI_escape_code#DOS,_OS/2,_and_Windows
//!     - Colors: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
//!   - ASCII control chars: https://www.asciitable.com/
//!   - VT100 Control codes: https://vt100.net/docs/vt100-ug/chapter3.html#ED
//! - ANSI (8-bit) vs ASCII (7-bit): http://www.differencebetween.net/technology/web-applications/difference-between-ansi-and-ascii/
//! - Windows Terminal (bash): https://www.makeuseof.com/windows-terminal-vs-powershell/

// Attach source files.
pub mod tw_color;
pub mod raw_mode;
pub mod tw_command;
pub mod input_event;
pub mod event_stream_ext;

// Re-export everything from attached source files.
pub use event_stream_ext::*;
pub use input_event::*;
pub use raw_mode::*;
pub use tw_color::*;
pub use tw_command::*;
