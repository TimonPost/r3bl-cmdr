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
use async_trait::async_trait;
use crossterm::event::*;

const DEBUG: bool = true;

/// Async trait object that implements the [Draw] trait.
#[derive(Default, Debug, Clone, Copy)]
pub struct AppNoLayout {
  pub lolcat: Lolcat,
}

#[async_trait]
impl Render<AppState, AppAction> for AppNoLayout {
  async fn render(
    &mut self,
    state: &AppState,
    _shared_store: &SharedStore<AppState, AppAction>,
    window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      let content = format!("{}", state);

      let content_size = content.len() as UnitType;
      let x: UnitType = window_size.width / 2 - content_size / 2;
      let y: UnitType = window_size.height / 2;

      let colored_content = colorize!(self, "{}", state);

      let queue = tw_queue!(
        TWCommand::ClearScreen,
        TWCommand::ResetColor,
        TWCommand::MoveCursorPosition((x, y)),
        TWCommand::PrintWithAttributes(colored_content, None),
        TWCommand::ResetColor
      );

      call_if_true!(DEBUG, {
        log_no_err!(INFO, "⛵ AppNoLayout::render -> size, state: {} {}", window_size, state);
        log_no_err!(INFO, "⛵ AppNoLayout::render -> queue: {}", queue);
      });
      queue
    });
  }

  async fn handle_event(
    &self,
    input_event: &InputEvent,
    _state: &AppState,
    shared_store: &SharedStore<AppState, AppAction>,
    _terminal_size: Size,
  ) -> CommonResult<()> {
    throws!({
      call_if_true!(
        DEBUG,
        log_no_err!(INFO, "⛵ AppNoLayout::handle_event -> input_event: {}", input_event)
      );

      if let InputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> + -> dispatch_spawn: {}",
                AppAction::AddPop(1)
              )
            );
          }
          '-' => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> - -> dispatch_spawn: {}",
                AppAction::SubPop(1)
              )
            );
          }
          _ => {}
        }
      }

      if let InputEvent::NonDisplayableKeypress(key_event) = input_event {
        match key_event {
          KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> Up -> dispatch_spawn: {}",
                AppAction::AddPop(1)
              )
            );
          }
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppNoLayout::handle_event -> Down -> dispatch_spawn: {}",
                AppAction::SubPop(1)
              )
            );
          }
          _ => {}
        }
      }
    });
  }
}
