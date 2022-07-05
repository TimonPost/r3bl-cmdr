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

use crossterm::event::{
  Event::{self, Key, Mouse, Resize},
  KeyCode, KeyEvent, KeyModifiers, MouseEvent,
};
use r3bl_rs_utils::*;
use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
  DisplayableKeypress(char),
  NonDisplayableKeypress(KeyEvent),
  Resize(Size),
  Mouse(MouseEvent),
  None,
}

/// For [ToString].
impl Display for InputEvent {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Default for InputEvent {
  fn default() -> Self {
    InputEvent::None
  }
}

/// Typecast / convert [Event] to [InputEvent].
impl From<Event> for InputEvent {
  fn from(event: Event) -> Self {
    match event {
      Key(key_event) => key_event.into(),
      Mouse(mouse_event) => mouse_event.into(),
      Resize(cols, rows) => (rows, cols).into(),
    }
  }
}

/// Typecast / convert [(u16, u16)] to [InputEvent::TerminalSize].
impl From<(/* rows: */ u16, /* cols: */ u16)> for InputEvent {
  fn from(size: (u16, u16)) -> Self {
    let (rows, cols) = size;
    InputEvent::Resize(Size {
      width: cols,
      height: rows,
    })
  }
}

/// Typecast / convert [MouseEvent] to [InputEvent::InputMouseEvent].
impl From<MouseEvent> for InputEvent {
  fn from(mouse_event: MouseEvent) -> Self {
    InputEvent::Mouse(mouse_event)
  }
}

/// Typecast / convert [KeyEvent] to [InputEvent::].
impl From<KeyEvent> for InputEvent {
  fn from(key_event: KeyEvent) -> Self {
    match key_event {
      // Check if "normal character" is pressed.
      KeyEvent {
        code: KeyCode::Char(character),
        modifiers: KeyModifiers::NONE,
      } => InputEvent::DisplayableKeypress(character),

      // All other key presses.
      _ => InputEvent::NonDisplayableKeypress(key_event),
    }
  }
}
