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

pub struct TerminalWindow {
  pub size: Size,
  pub stream: EventStream,
}

impl TerminalWindow {
  fn try_to_create_instance() -> CommonResult<Self> {
    Ok(Self {
      stream: EventStream::new(),
      size: Size::try_to_get_from_crossterm_terminal()?,
    })
  }

  /// The where clause needs to match up w/ the trait bounds for [Store].
  ///
  /// ```ignore
  /// where
  /// S: Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  /// A: Default + Clone + Sync + Send,
  /// ```
  pub async fn start_event_loop<S, A>(
    store: Store<S, A>,
    shared_render: SharedRender<S, A>,
  ) -> CommonResult<()>
  where
    S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
    A: Display + Default + Clone + Sync + Send,
  {
    raw_mode!({
      // Initialize the terminal window struct.
      let _window = TerminalWindow::try_to_create_instance()?;
      let shared_window: SharedWindow = Arc::new(RwLock::new(_window));

      // Move the store into an Arc & RwLock.
      let shared_store: SharedStore<S, A> = Arc::new(RwLock::new(store));

      // Create a subscriber & perform the first render.
      let subscriber = MySubscriber::new_box(&shared_render, &shared_store, &shared_window);
      subscriber.run(shared_store.read().await.get_state().await).await;

      // Attach a subscriber to the store.
      shared_store.write().await.add_subscriber(subscriber).await;

      call_if_true!(DEBUG, shared_window.read().await.log_state("Startup"));

      // Main event loop.
      loop {
        // Try and get the next event if available (asynchronously).
        let maybe_input_event = shared_window.write().await.stream.try_to_get_input_event().await;

        // Process the input_event.
        if let Some(input_event) = maybe_input_event {
          call_if_true!(DEBUG, log_no_err!(INFO, "Tick: ⏰ {}", input_event));
          if let Continuation::Exit = base_handle_event(input_event, &shared_window).await {
            break;
          }
          let my_state = shared_store.read().await.get_state().await;
          let window_size = shared_window.read().await.size;
          shared_render
            .read()
            .await
            .handle_event(&input_event, &my_state, &shared_store, window_size)
            .await?;
        }

        // This flush command is needed in order to keep stdout in sync w/ the event loop.
        TWCommand::flush();
      }
    })
  }

  /// Dump the state of the terminal window to the log.
  pub fn log_state(&self, msg: &str) {
    log_no_err!(INFO, "{} -> {}", msg, self.to_string());
  }
}

struct MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send + 'static,
  A: Display + Default + Clone + Sync + Send + 'static,
{
  shared_draw: SharedRender<S, A>,
  shared_store: SharedStore<S, A>,
  shared_window: SharedWindow,
}

#[async_trait]
impl<S, A> AsyncSubscriber<S> for MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  async fn run(&self, state: S) {
    let window_size = self.shared_window.read().await.size;
    let render_result = self
      .shared_draw
      .read()
      .await
      .render(&state, &self.shared_store, window_size)
      .await;
    match render_result {
      Err(error) => {
        log_no_err!(ERROR, "MySubscriber::run draw error: {}", error);
        TWCommand::flush();
      }
      Ok(mut command_queue) => {
        command_queue.flush();
      }
    }
  }
}

impl<S, A> MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  fn new_box(
    shared_draw: &SharedRender<S, A>,
    shared_store: &SharedStore<S, A>,
    shared_window: &SharedWindow,
  ) -> Box<Self> {
    Box::new(MySubscriber {
      shared_draw: shared_draw.clone(),
      shared_store: shared_store.clone(),
      shared_window: shared_window.clone(),
    })
  }
}
