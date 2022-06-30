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
  sync::Arc,
};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct TWData {
  pub size: Size,
}

impl TWData {
  fn try_to_create_instance() -> CommonResult<TWData> {
    Ok(TWData {
      size: Size::try_to_get_from_crossterm_terminal()?,
    })
  }

  pub fn log_state(&self, msg: &str) {
    log_no_err!(INFO, "{} -> {:?}", msg, self);
  }

  pub fn set_size(&mut self, new_size: Size) {
    self.size = new_size;
    call_if_true!(DEBUG, self.log_state("main_event_loop -> Resize"));
  }
}

pub struct TerminalWindow;

impl TerminalWindow {
  /// The where clause needs to match up w/ the trait bounds for [Store].
  ///
  /// ```ignore
  /// where
  /// S: Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  /// A: Default + Clone + Sync + Send,
  /// ```
  pub async fn main_event_loop<S, A>(store: Store<S, A>, shared_render: SharedRender<S, A>, exit_keys: Vec<KeyEvent>) -> CommonResult<()>
  where
    S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send + 'static,
    A: Display + Default + Clone + Sync + Send + 'static,
  {
    raw_mode!({
      // Initialize the terminal window data struct.
      let _tw_data = TWData::try_to_create_instance()?;
      let shared_window: SharedWindow = Arc::new(RwLock::new(_tw_data));

      // Move the store into an Arc & RwLock.
      let shared_store: SharedStore<S, A> = Arc::new(RwLock::new(store));

      // Create a subscriber & attach it to the store.
      let _subscriber = TWSubscriber::new_box(&shared_render, &shared_store, &shared_window);
      shared_store.write().await.add_subscriber(_subscriber).await;

      // Create a new event stream (async).
      let mut stream = EventStream::new();

      // Perform first render.
      TWSubscriber::render(&shared_store, &shared_render, shared_window.read().await.size, None).await?;

      call_if_true!(DEBUG, shared_window.read().await.log_state("main_event_loop -> Startup 🚀"));

      // Main event loop.
      loop {
        // Try and get the next event if available (asynchronously).
        let maybe_input_event = stream.try_to_get_input_event().await;

        // Process the input_event.
        if let Some(input_event) = maybe_input_event {
          call_if_true!(DEBUG, log_no_err!(INFO, "main_event_loop -> Tick: ⏰ {}", input_event));

          match DefaultInputEventHandler::no_consume(input_event, &exit_keys).await {
            Continuation::Exit => {
              break;
            }
            Continuation::ResizeAndContinue(new_size) => {
              shared_window.write().await.set_size(new_size);
              TWSubscriber::render(&shared_store, &shared_render, new_size, None).await?;
            }
            Continuation::Continue => TWSubscriber::handle_input(&shared_window, &shared_store, &shared_render, &input_event).await?,
          };
        }
        TWCommand::flush();
      }
    })
  }
}

struct TWSubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send + 'static,
  A: Display + Default + Clone + Sync + Send + 'static,
{
  shared_render: SharedRender<S, A>,
  shared_store: SharedStore<S, A>,
  shared_window: SharedWindow,
}

#[async_trait]
impl<S, A> AsyncSubscriber<S> for TWSubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  async fn run(&self, my_state: S) {
    let window_size = self.shared_window.read().await.size;
    let result = TWSubscriber::render(&self.shared_store, &self.shared_render, window_size, Some(my_state)).await;
    if let Err(e) = result {
      log_no_err!(ERROR, "MySubscriber::run -> Error: {}", e);
    }
  }
}

impl<S, A> TWSubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  fn new_box(shared_draw: &SharedRender<S, A>, shared_store: &SharedStore<S, A>, shared_window: &SharedWindow) -> Box<Self> {
    Box::new(TWSubscriber {
      shared_render: shared_draw.clone(),
      shared_store: shared_store.clone(),
      shared_window: shared_window.clone(),
    })
  }

  /// Pass the event to the shared_render for further processing.
  pub async fn handle_input(
    shared_window: &SharedWindow,
    shared_store: &SharedStore<S, A>,
    shared_render: &SharedRender<S, A>,
    input_event: &InputEvent,
  ) -> CommonResult<()> {
    throws!({
      let latest_state = shared_store.read().await.get_state();
      let window_size = shared_window.read().await.size;
      shared_render
        .read()
        .await
        .handle_event(input_event, &latest_state, &shared_store, window_size)
        .await?
    });
  }

  pub async fn render(
    shared_store: &SharedStore<S, A>,
    shared_render: &SharedRender<S, A>,
    window_size: Size,
    my_state: Option<S>,
  ) -> CommonResult<()> {
    throws!({
      let state: S = if my_state.is_none() {
        shared_store.read().await.get_state()
      } else {
        my_state.unwrap()
      };

      let render_result = shared_render.write().await.render(&state, &shared_store, window_size).await;
      match render_result {
        Err(error) => {
          TWCommand::flush();
          call_if_true!(DEBUG, log_no_err!(ERROR, "MySubscriber::run draw error: {}", error));
        }
        Ok(command_queue) => {
          command_queue.flush();
          call_if_true!(DEBUG, log_no_err!(INFO, "MySubscriber::run draw: {}, {}", window_size, state));
        }
      }
    });
  }
}
