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

/// Check to see if the [InputEvent] matches one of the [EXIT_KEYS] & if so, return
/// [Continuation::Exit]. Otherwise, do some work w/ the `input_event` & return
/// [Continuation::Continue].
///
/// Note that [InputEvent] implements [Copy] (no need to pass references into this
/// function).
pub async fn base_handle_event(input_event: InputEvent, window: &SharedWindow) -> Continuation {
  // Early return if any exit key sequence is pressed.
  if let Continuation::Exit = input_event.into() {
    return Continuation::Exit;
  }

  // Default input event handling.
  match input_event {
    InputEvent::NonDisplayableKeypress(key_event) => {
      log_no_err!(INFO, "NonDisplayableKeypress: {:?}", key_event);
    }
    InputEvent::DisplayableKeypress(character) => {
      log_no_err!(INFO, "DisplayableKeypress: {:?}", character);
    }
    InputEvent::Resize(size) => {
      on_resize(size, window).await;
    }
    InputEvent::Mouse(mouse_event) => {
      log_no_err!(INFO, "Mouse: {:?}", mouse_event);
    }
    _ => {
      log_no_err!(INFO, "Other: {:?}", input_event);
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

#[non_exhaustive]
pub enum Continuation {
  Exit,
  Continue,
}

async fn on_resize(size: Size, window: &SharedWindow) {
  window.write().await.size = size;
  log_no_err!(INFO, "Resize: {:?}", (size.height, size.width));
  call_if_true!(DEBUG, window.read().await.log_state("Resize"));
}
