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

use crate::*;
use crossterm::{
  cursor::*,
  event::*,
  style::*,
  terminal::{self, *},
  *,
};
use lazy_static::lazy_static;
use r3bl_rs_utils::*;
use serde::{Deserialize, Serialize};

use std::{
  collections::HashMap,
  fmt::{Display, Formatter, Result},
  io::{stderr, stdout, Write},
  ops::AddAssign,
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
          call_if_true!(DEBUG, log!(ERROR, "crossterm: ❌ Failed to {} due to {}", $msg, err));
        } else {
          call_if_true!(DEBUG, log!(INFO, "crossterm: ✅ {} successfully", $msg));
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

/// This works together w/ [TWCommand] to enqueue commands, but not flush them. It will
/// return a [TWCommandQueue]. Here's an example.
///
/// ```ignore
/// let mut queue = queue!(
///   TWCommand::ClearScreen,
///   TWCommand::ResetColor
/// );
/// ```
///
/// Decl macro docs:
/// - https://veykril.github.io/tlborm/decl-macros/macros-methodical.html#repetitions
#[macro_export]
macro_rules! tw_queue {
  (
    $(                      /* Start a repetition. */
      $element:expr         /* Expression. */
    )                       /* End repetition. */
    ,                       /* Comma separated. */
    *                       /* Zero or more times. */
  ) => {
    /* Enclose the expansion in a block so that we can use multiple statements. */
    {
      let mut queue = TWCommandQueue::default();
      /* Start a repetition. */
      $(
        /* Each repeat will contain the following statement, with $element replaced. */
        queue.push($element);
      )*
      queue
    }
  };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum TWCommand {
  EnterRawMode,
  ExitRawMode,
  /// (column, row).
  MoveCursorPosition((UnitType, UnitType)),
  ClearScreen,
  SetFgColor(TWColor),
  SetBgColor(TWColor),
  ResetColor,
  ApplyColors(Option<Style>),
  PrintWithAttributes(String, Option<Style>),
  CursorShow,
  CursorHide,
}

impl TWCommand {
  pub fn flush() {
    exec!(stdout().flush(), "flush stdout");
    exec!(stderr().flush(), "flush stderr");
  }
}

/// This works w/ [TWCommand] items. It allows them to be added in sequence, and then
/// flushed at the end. Here's an example.
/// ```ignore
/// let mut queue = CommandQueue::default();
/// queue.add(TWCommand::ClearScreen);
/// queue.add(TWCommand::CursorShow);
/// queue.flush();
/// ```
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TWCommandQueue {
  pub queue: Vec<TWCommand>,
}

impl Display for TWCommandQueue {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for cmd in &self.queue {
      writeln!(f, "{:?}", cmd)?;
    }
    Ok(())
  }
}

impl AddAssign for TWCommandQueue {
  fn add_assign(&mut self, other: TWCommandQueue) {
    self.queue.extend(other.queue);
  }
}

impl TWCommandQueue {
  pub fn push(&mut self, cmd_wrapper: TWCommand) {
    self.queue.push(cmd_wrapper);
  }

  #[allow(unreachable_patterns)]
  pub fn flush(&self) {
    let mut skip_flush = false;

    self.queue.iter().for_each(|cmd_wrapper| match cmd_wrapper {
      TWCommand::EnterRawMode => {
        exec! {
          terminal::enable_raw_mode(),
          "EnterRawMode -> enable_raw_mode()"
        };
        exec! {
          queue!(stdout(),
            EnableMouseCapture,
            EnterAlternateScreen,
            MoveTo(0,0),
            Clear(ClearType::All),
            Hide,
          ),
        "EnterRawMode -> EnableMouseCapture, EnterAlternateScreen, MoveTo(0,0), Clear(ClearType::All), Hide"
        }
        TWCommand::flush();
        skip_flush = true;
      }
      TWCommand::ExitRawMode => {
        exec! {
          queue!(stdout(),
            Show,
            LeaveAlternateScreen,
            DisableMouseCapture
          ),
          "ExitRawMode -> Show, LeaveAlternateScreen, DisableMouseCapture"
        };
        TWCommand::flush();
        exec! {terminal::disable_raw_mode(), "ExitRawMode -> disable_raw_mode()"}
        skip_flush = true;
      }
      TWCommand::MoveCursorPosition((col, row)) => {
        exec!(
          queue!(stdout(), MoveTo(*col, *row)),
          format!("MoveCursorPosition(col: {}, row: {})", *col, *row)
        )
      }
      TWCommand::ClearScreen => {
        exec!(queue!(stdout(), Clear(ClearType::All)), "ClearScreen")
      }
      TWCommand::SetFgColor(color) => {
        exec!(
          queue!(stdout(), SetForegroundColor(**color)),
          format!("SetFgColor({:?})", color)
        )
      }
      TWCommand::SetBgColor(color) => {
        exec!(
          queue!(stdout(), SetBackgroundColor(**color)),
          format!("SetBgColor({:?})", color)
        )
      }
      TWCommand::ResetColor => {
        exec!(queue!(stdout(), ResetColor), "ResetColor")
      }
      TWCommand::CursorShow => {
        exec!(queue!(stdout(), Show), "CursorShow")
      }
      TWCommand::CursorHide => {
        exec!(queue!(stdout(), Hide), "CursorHide")
      }
      TWCommand::ApplyColors(style) => {
        if style.is_some() {
          // Use Style to set crossterm Colors.
          // Docs: https://docs.rs/crossterm/latest/crossterm/style/index.html#colors
          let mut style = style.clone().unwrap();
          let mask = style.get_bitflags();
          if mask.contains(StyleFlag::COLOR_BG_SET) {
            let color_bg = style.color_bg.unwrap();
            exec!(
              queue!(stdout(), SetBackgroundColor(*color_bg)),
              format!("ApplyColors -> SetBackgroundColor({:?})", *color_bg)
            )
          }
          if mask.contains(StyleFlag::COLOR_FG_SET) {
            let color_fg = style.color_fg.unwrap();
            exec!(
              queue!(stdout(), SetForegroundColor(*color_fg)),
              format!("ApplyColors -> SetForegroundColor({:?})", *color_fg)
            )
          }
        }
      }
      TWCommand::PrintWithAttributes(text, style) => {
        if style.is_some() {
          // Use Style to set crossterm Attributes.
          // Docs: https://docs.rs/crossterm/latest/crossterm/style/index.html#attributes
          let mut style = style.clone().unwrap();
          let mask = style.get_bitflags();
          let mut needs_reset = false;

          STYLE_TO_ATTRIBUTE_MAP.iter().for_each(|(flag, attr)| {
            if mask.contains(*flag) {
              exec!(
                queue!(stdout(), SetAttribute(*attr)),
                format!("PrintWithAttributes -> SetAttribute({:?})", attr)
              );
              needs_reset = true;
            }
          });

          exec!(
            queue!(stdout(), Print(text.clone())),
            format!("PrintWithAttributes -> Print({:?})", text)
          );

          if needs_reset {
            exec!(
              queue!(stdout(), SetAttribute(Attribute::Reset)),
              format!("PrintWithAttributes -> SetAttribute(Reset))")
            );
          }
        } else {
          exec!(
            queue!(stdout(), Print(text.clone())),
            format!("PrintWithAttributes -> Print({:?})", text)
          )
        }
      }
      _ => {
        unimplemented!("TWCommandQueue::flush() 🧨 {:?} not implemented", cmd_wrapper)
      }
    });

    // Flush all the commands that were added via calls to `queue!` above.
    if !skip_flush {
      TWCommand::flush()
    };
  }
}

lazy_static! {
  static ref STYLE_TO_ATTRIBUTE_MAP: HashMap<StyleFlag, Attribute> = {
    let mut map = HashMap::new();
    map.insert(StyleFlag::BOLD_SET, Attribute::Bold);
    map.insert(StyleFlag::DIM_SET, Attribute::Dim);
    map.insert(StyleFlag::UNDERLINE_SET, Attribute::Underlined);
    map.insert(StyleFlag::REVERSE_SET, Attribute::Reverse);
    map.insert(StyleFlag::HIDDEN_SET, Attribute::Hidden);
    map.insert(StyleFlag::STRIKETHROUGH_SET, Attribute::Fraktur);
    map
  };
}
