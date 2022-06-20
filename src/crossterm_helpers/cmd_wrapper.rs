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
  cursor::{self},
  event::*,
  queue,
  terminal::{self, *},
};
use paste::paste;
use r3bl_rs_utils::*;
use std::io::{stdout, Write};

/// Given a crossterm command, this will run it and [log!] the [Result] that is returned.
/// If [log!] fails, then it will print a message to stderr.
///
/// Paste docs: https://github.com/dtolnay/paste
#[macro_export]
macro_rules! try_to_run_crossterm_command_and_log_result {
  ($cmd: expr, $name: expr) => {{
    paste! {
      // Generate a new function that returns [CommonResult].
      let [<_ $name>] = || -> CommonResult<()> {
        throws!({
          if let Err(err) = $cmd {
            log!(ERROR, "crossterm: ❌ Failed to {} due to {}", $name, err);
          } else {
            log!(INFO, "crossterm: ✅ {} successfully", $name);
          }
        })
      };

      // Call this generated function. It will fail if there are problems w/ log!().
      // In this case, if DEBUG is true, then it will dump the error to stderr.
      if let Err(err) = [<_ $name>]() {
        let msg = format!("❌ Failed to {}", $name);
        call_if_true!(DEBUG,
          debug!(ERROR_RAW &msg, err)
        );
      }
    }
  }};
}

/// This works together w/ [Command] to enqueue commands, and then flush them at the end.
/// Here's an example.
/// ```ignore
/// queue_and_flush!(
///   Command::EnableRawMode,
///   Command::EnableMouseCapture,
///   Command::EnterAlternateScreen,
///   Command::ResetCursorPosition,
///   Command::ClearScreen
/// );
/// ```
/// Decl macro docs: https://veykril.github.io/tlborm/decl-macros/macros-methodical.html#repetitions
#[macro_export]
macro_rules! queue_and_flush {
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
          queue.flush();
      }
  };
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Command {
  EnableRawMode,
  EnableMouseCapture,
  EnterAlternateScreen,
  LeaveAlternateScreen,
  DisableRawMode,
  DisableMouseCapture,
  MoveCursorPosition(UnitType, UnitType),
  ClearScreen,
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
  pub queue: Vec<Command>,
}

impl CommandQueue {
  pub fn add(&mut self, cmd_wrapper: Command) {
    self.queue.push(cmd_wrapper);
  }

  pub fn flush(&mut self) {
    for cmd_wrapper in &mut self.queue {
      match cmd_wrapper {
        Command::EnableRawMode => try_to_run_crossterm_command_and_log_result! {
          terminal::enable_raw_mode(),
          "enable_raw_mode"
        },
        Command::EnableMouseCapture => try_to_run_crossterm_command_and_log_result! {
          queue!(stdout(), EnableMouseCapture),
          "enable_mouse_capture"
        },
        Command::EnterAlternateScreen => try_to_run_crossterm_command_and_log_result! {
          queue!(stdout(), EnterAlternateScreen),
          "enter_alternate_screen"
        },
        Command::LeaveAlternateScreen => try_to_run_crossterm_command_and_log_result! {
          queue!(stdout(), LeaveAlternateScreen),
          "leave_alternate_screen"
        },
        Command::DisableRawMode => try_to_run_crossterm_command_and_log_result! {
          terminal::disable_raw_mode(),
          "disable_raw_mode"
        },
        Command::DisableMouseCapture => try_to_run_crossterm_command_and_log_result! {
          queue!(stdout(), DisableMouseCapture) ,
          "disable_mouse_mode"
        },
        Command::MoveCursorPosition(first, second) => {
          let msg = format!("move_to: {}, {}", first, second);
          try_to_run_crossterm_command_and_log_result! {
            queue!(stdout(), cursor::MoveTo(*first, *second)),
            msg
          }
        }
        Command::ClearScreen => try_to_run_crossterm_command_and_log_result! {
          queue!(stdout(), terminal::Clear(ClearType::All)),
          "clear_screen"
        },
      };
    }

    try_to_run_crossterm_command_and_log_result! {
      stdout().flush(),
      "flush"
    }
  }
}
