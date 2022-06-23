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
use crossterm::event::*;
use r3bl_rs_utils::*;

/// Array of [KeyEvent]s that the user can press to exit the REPL.
const EXIT_KEYS: [crossterm::event::KeyEvent; 1] = [KeyEvent {
  code: KeyCode::Char('q'),
  modifiers: KeyModifiers::CONTROL,
}];

#[non_exhaustive]
pub enum Continuation {
  Exit,
  Continue,
  ResizeAndContinue(Size),
}

/// This function does **not** consume the `input_event` argument. [InputEvent] implements
/// [Copy] (no need to pass references into this function).
pub async fn handle_event_no_consume(input_event: InputEvent) -> Continuation {
  // Early return if any exit key sequence is pressed.
  if let Continuation::Exit = input_event.into() {
    return Continuation::Exit;
  }

  // Default input event handling.
  match input_event {
    InputEvent::NonDisplayableKeypress(key_event) => {
      call_if_true!(
        DEBUG,
        log_no_err!(INFO, "default_event_handler -> NonDisplayableKeypress: {:?}", key_event)
      );
    }
    InputEvent::DisplayableKeypress(character) => {
      call_if_true!(DEBUG, log_no_err!(INFO, "default_event_handler -> DisplayableKeypress: {:?}", character));
    }
    InputEvent::Resize(size) => {
      call_if_true!(DEBUG, log_no_err!(INFO, "default_event_handler -> Resize: {:?}", size));
      return Continuation::ResizeAndContinue(size);
    }
    InputEvent::Mouse(mouse_event) => {
      call_if_true!(DEBUG, log_no_err!(INFO, "default_event_handler -> Mouse: {:?}", mouse_event));
    }
    _ => {
      call_if_true!(DEBUG, log_no_err!(INFO, "default_event_handler -> Other: {:?}", input_event));
    }
  }

  Continuation::Continue
}

impl From<InputEvent> for Continuation {
  /// Convert [InputEvent] to [Continuation]. This checks whether the [InputEvent] matches
  /// any of the [EXIT_KEYS] & returns [Continuation::Exit]. Otherwise returns a
  /// [Continuation::Continue].
  fn from(input_event: InputEvent) -> Self {
    if let InputEvent::NonDisplayableKeypress(key_event) = input_event {
      if EXIT_KEYS.contains(&key_event) {
        return Continuation::Exit;
      }
    }
    Continuation::Continue
  }
}
