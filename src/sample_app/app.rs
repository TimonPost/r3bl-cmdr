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
use crossterm::event::*;

/// Async trait object that implements the [Draw] trait.
#[derive(Default)]
pub struct App;

#[async_trait]
impl Render<State, Action> for App {
  async fn render(
    &self,
    state: &State,
    _shared_store: &SharedStore<State, Action>,
    _terminal_size: Size,
  ) -> CommonResult<CommandQueue> {
    throws_with_return!({
      let mut queue = CommandQueue::default();

      {
        // TODO: actually generate some commands to be rendered
        // TODO: remove debug
        println!("⛵ Draw -> draw: {}\r", state);
      }

      queue
    });
  }

  async fn handle_event(
    &self,
    input_event: &InputEvent,
    _state: &State,
    shared_store: &SharedStore<State, Action>,
    _terminal_size: Size,
  ) -> CommonResult<()> {
    throws!({
      match input_event {
        InputEvent::DisplayableKeypress(typed_char) => match typed_char {
          '+' => shared_store.read().await.dispatch_spawn(Action::AddPop(1)),
          '-' => shared_store.read().await.dispatch_spawn(Action::SubPop(1)),
          _ => {}
        },
        InputEvent::NonDisplayableKeypress(key_event) => match key_event {
          KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
          } => shared_store.read().await.dispatch_spawn(Action::AddPop(1)),
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => shared_store.read().await.dispatch_spawn(Action::SubPop(1)),
          _ => {}
        },
        _ => {}
      }
    });
  }
}
