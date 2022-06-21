/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use crate::*;
use crossterm::{
  cursor::{self, *},
  event::*,
  style::{self, *},
  terminal::{self, *},
  *,
};
use r3bl_rs_utils::*;
use std::{
  fmt::Display,
  io::{stderr, stdout, Write},
};

/// Given a crossterm command, this will run it and [log!] the [Result] that is returned.
/// If [log!] fails, then it will print a message to stderr.
///
/// Paste docs: https://github.com/dtolnay/paste
#[macro_export]
macro_rules! exec {
  ($cmd: expr, $msg: expr) => {{
    // Generate a new function that returns [CommonResult].
    let _new_fn_name = || -> CommonResult<()> {
      throws!({
        if let Err(err) = $cmd {
          log!(ERROR, "crossterm: ❌ Failed to {} due to {}", $msg, err);
        } else {
          log!(INFO, "crossterm: ✅ {} successfully", $msg);
        }
      })
    };

    // Call this generated function. It will fail if there are problems w/ log!().
    // In this case, if DEBUG is true, then it will dump the error to stderr.
    if let Err(err) = _new_fn_name() {
      let msg = format!("❌ Failed to {}", $msg);
      call_if_true!(DEBUG, debug!(ERROR_RAW & msg, err));
    }
  }};
}

/// This works together w/ [Command] to enqueue commands, but not flush them. It will
/// return a [CommandQueue]. Here's an example.
///
/// ```ignore
/// let mut queue = queue!(
///   Command::EnableRawMode,
///   Command::EnableMouseCapture,
///   Command::EnterAlternateScreen,
///   Command::ResetCursorPosition,
///   Command::ClearScreen
/// );
/// ```
///
/// Decl macro docs:
/// - https://veykril.github.io/tlborm/decl-macros/macros-methodical.html#repetitions
#[macro_export]
macro_rules! tw_queue {
    (
      // Start a repetition:
      $(
          // Each repeat must contain an expression...
          $element:expr
      )
      // ...separated by commas...
      ,
      // ...zero or more times.
      *
  ) => {
      // Enclose the expansion in a block so that we can use
      // multiple statements.
      {
          let mut queue = CommandQueue::default();
          // Start a repetition:
          $(
              // Each repeat will contain the following statement, with
              // $element replaced with the corresponding expression.
              queue.add($element);
          )*
          queue
      }
  };
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TWCommand {
  EnableRawMode,
  EnableMouseCapture,
  EnterAlternateScreen,
  LeaveAlternateScreen,
  DisableRawMode,
  DisableMouseCapture,
  MoveCursorPosition(UnitType, UnitType),
  ClearScreen,
  SetFgColor(Color),
  SetBgColor(Color),
  ResetColor,
  Print(String),
  CursorShow,
  CursorHide,
}

impl TWCommand {
  pub fn flush() {
    exec!(stdout().flush(), "flush stdout");
    exec!(stderr().flush(), "flush stderr");
  }
}

/// This works w/ [Command] items. It allows them to be added in sequence, and then
/// flushed at the end. Here's an example.
/// ```ignore
/// let mut queue = CommandQueue::default();
/// queue.add(Command::EnableRawMode);
/// queue.add(Command::EnableMouseCapture);
/// queue.add(Command::EnterAlternateScreen);
/// queue.add(Command::ResetCursorPosition);
/// queue.add(Command::ClearScreen);
/// queue.flush();
/// ```
#[derive(Default, Debug)]
pub struct CommandQueue {
  pub queue: Vec<TWCommand>,
}

impl Display for CommandQueue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl CommandQueue {
  pub fn add(&mut self, cmd_wrapper: TWCommand) {
    self.queue.push(cmd_wrapper);
  }

  pub fn flush(&mut self) {
    self.queue.iter().for_each(|cmd_wrapper| match cmd_wrapper {
      TWCommand::EnableRawMode => {
        exec!(terminal::enable_raw_mode(), "EnableRawMode")
      }
      TWCommand::EnableMouseCapture => {
        exec!(queue!(stdout(), EnableMouseCapture), "EnableMouseCapture")
      }
      TWCommand::EnterAlternateScreen => {
        exec!(queue!(stdout(), EnterAlternateScreen), "EnterAlternateScreen")
      }
      TWCommand::LeaveAlternateScreen => {
        exec!(queue!(stdout(), LeaveAlternateScreen), "LeaveAlternateScreen")
      }
      TWCommand::DisableRawMode => {
        exec!(terminal::disable_raw_mode(), "DisableRawMode")
      }
      TWCommand::DisableMouseCapture => {
        exec!(queue!(stdout(), DisableMouseCapture), "DisableMouseCapture")
      }
      TWCommand::MoveCursorPosition(first, second) => {
        exec!(
          queue!(stdout(), cursor::MoveTo(*first, *second)),
          format!("MoveCursorPosition({}, {})", first, second)
        )
      }
      TWCommand::ClearScreen => {
        exec!(queue!(stdout(), terminal::Clear(ClearType::All)), "ClearScreen")
      }
      TWCommand::SetFgColor(color) => {
        exec!(
          queue!(stdout(), style::SetForegroundColor(*color)),
          format!("SetFgColor({:?})", color)
        )
      }
      TWCommand::SetBgColor(color) => {
        exec!(
          queue!(stdout(), style::SetBackgroundColor(*color)),
          format!("SetBgColor({:?})", color)
        )
      }
      TWCommand::ResetColor => {
        exec!(queue!(stdout(), style::ResetColor), "ResetColor")
      }
      TWCommand::Print(content) => {
        exec!(
          queue!(stdout(), style::Print(content.clone())),
          format!("Print({:?})", content)
        )
      }
      TWCommand::CursorShow => {
        exec!(queue!(stdout(), Show), "CursorShow")
      }
      TWCommand::CursorHide => {
        exec!(queue!(stdout(), Hide), "CursorHide")
      }
    });

    TWCommand::flush();
  }
}
