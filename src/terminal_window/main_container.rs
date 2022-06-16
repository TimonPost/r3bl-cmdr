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
use r3bl_rs_utils::*;
use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

pub struct TerminalWindow {
  pub terminal_size: Size,
  pub event_stream: EventStream,
}

impl TerminalWindow {
  fn try_to_create_instance() -> CommonResult<Self> {
    Ok(Self {
      event_stream: EventStream::new(),
      terminal_size: Size::try_to_get_from_crossterm_terminal()?,
    })
  }

  /// The where clause needs to match up w/ the trait bounds for [Store].
  ///
  /// ```ignore
  /// where
  /// S: Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  /// A: Default + Clone + Sync + Send,
  /// ```
  pub async fn start_event_loop<S, A>(store: &mut Store<S, A>, shared_draw: ShareDraw<S>) -> CommonResult<()>
  where
    S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
    A: Display + Default + Clone + Sync + Send,
  {
    raw_mode!({
      let mut terminal_window = TerminalWindow::try_to_create_instance()?;

      // Attach a subscriber to the store.
      let subscriber = TerminalWindowSubscriber {
        shared_draw: shared_draw.clone(),
      };
      subscriber.run(store.get_state().await).await;
      store.add_subscriber(Box::new(subscriber)).await;

      // Clone the shared_draw for use in the loop below.
      let my_draw_clone: ShareDraw<S> = shared_draw.clone();

      call_if_true!(DEBUG, terminal_window.dump_state_to_log("Startup"));

      loop {
        let maybe_input_event = terminal_window.event_stream.get_input_event().await;
        if let Some(input_event) = maybe_input_event {
          let loop_continuation = handle_input_event(input_event, &mut terminal_window).await;
          if let LoopContinuation::Exit = loop_continuation {
            break;
          } else {
            // TODO: refactor this block into a function & add docs
            // TODO: replace this w/ functioning logic
            let my_rl_draw = my_draw_clone.read().await;
            let my_state = store.get_state().await;
            my_rl_draw.draw(&my_state).await?;
            // TODO: pass a store to handle_event()
            my_rl_draw.handle_event(&input_event, &my_state).await?;
          }
        }
      }
    })
  }

  /// Dump the state of the terminal window to the log.
  pub fn dump_state_to_log(&self, msg: &str) {
    log_no_err!(INFO, "{} -> {}", msg, self.to_string());
  }
}

// TODO: add docs
struct TerminalWindowSubscriber<S> {
  shared_draw: ShareDraw<S>,
}

#[async_trait]
impl<S> AsyncSubscriber<S> for TerminalWindowSubscriber<S>
where
  S: Display + Send + Sync + 'static,
{
  async fn run(&self, state: S) {
    println!("subscriber: {}", state);
    let my_draw_clone: ShareDraw<S> = self.shared_draw.clone();
    let my_rl_draw = my_draw_clone.read().await;
    let draw_result = my_rl_draw.draw(&state).await;
    if let Err(e) = draw_result {
      log_no_err!(ERROR, "TerminalWindowSubscriber::run draw error: {}", e);
    }
  }
}
