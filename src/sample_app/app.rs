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
use async_trait::async_trait;
use crossterm::{event::*, style::*};

const DEBUG: bool = true;

/// Async trait object that implements the [Draw] trait.
#[derive(Default)]
pub struct App;

#[async_trait]
impl Render<AppState, AppAction> for App {
  async fn render(
    &self,
    state: &AppState,
    _shared_store: &SharedStore<AppState, AppAction>,
    window_size: Size,
  ) -> CommonResult<CommandQueue> {
    throws_with_return!({
      let content = format!("{}", state);
      let content_size = content.len() as UnitType;
      let x: UnitType = window_size.width / 2 - content_size / 2;
      let y: UnitType = window_size.height / 2;

      let state_stack_top: i32 = *state.stack.last().unwrap();
      let green_color_value: u8 = if state_stack_top < 0 {
        255
      } else if state_stack_top > 255 {
        0
      } else {
        state_stack_top as u8
      };

      let queue = tw_queue!(
        TWCommand::ClearScreen,
        TWCommand::ResetColor,
        TWCommand::MoveCursorPosition(x, y),
        TWCommand::SetFgColor(Color::Rgb {
          r: if state_stack_top < 0 { 100 } else { 200 },
          g: green_color_value,
          b: if state_stack_top < 0 { 100 } else { 200 },
        }),
        TWCommand::Print(content),
        TWCommand::ResetColor
      );

      call_if_true!(DEBUG, {
        log_no_err!(INFO, "⛵ App::render -> size, state: {} {}", window_size, state);
        log_no_err!(INFO, "⛵ App::render -> queue: {}", queue);
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
        log_no_err!(INFO, "⛵ App::handle_event -> input_event: {}", input_event)
      );

      if let InputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ App::handle_event -> + -> dispatch_spawn: {}",
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
                "⛵ App::handle_event -> - -> dispatch_spawn: {}",
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
                "⛵ App::handle_event -> Up -> dispatch_spawn: {}",
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
                "⛵ App::handle_event -> Down -> dispatch_spawn: {}",
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
