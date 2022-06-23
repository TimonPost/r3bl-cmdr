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

#[derive(Clone)]
pub struct TerminalWindow {
  pub size: Size,
}

impl TerminalWindow {
  fn try_to_create_instance() -> CommonResult<Self> {
    Ok(Self {
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
  pub async fn main_event_loop<S, A>(
    store: Store<S, A>,
    shared_render: SharedRender<S, A>,
  ) -> CommonResult<()>
  where
    S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
    A: Display + Default + Clone + Sync + Send,
  {
    raw_mode!({
      // Initialize the terminal window data struct.
      let tw_data = TerminalWindow::try_to_create_instance()?;
      let init_size = tw_data.size;
      let shared_window: SharedWindow = Arc::new(RwLock::new(tw_data));

      // Move the store into an Arc & RwLock.
      let shared_store: SharedStore<S, A> = Arc::new(RwLock::new(store));

      // Create a subscriber.
      let subscriber = MySubscriber::new_box(&shared_render, &shared_store, &shared_window);

      // Attach a subscriber to the store.
      shared_store.write().await.add_subscriber(subscriber).await;

      // Create a new event stream (async).
      let mut stream = EventStream::new();

      // Perform first render.
      perform_render(&shared_store, &shared_render, init_size).await?;

      call_if_true!(
        DEBUG,
        shared_window
          .read()
          .await
          .log_state("main_event_loop -> Startup 🚀")
      );

      // Main event loop.
      loop {
        // Try and get the next event if available (asynchronously).
        let maybe_input_event = stream.try_to_get_input_event().await;

        // Process the input_event.
        if let Some(input_event) = maybe_input_event {
          call_if_true!(
            DEBUG,
            log_no_err!(INFO, "main_event_loop -> Tick: ⏰ {}", input_event)
          );

          match handle_event_no_consume(input_event).await {
            Continuation::Exit => {
              break;
            }
            Continuation::ResizeAndContinue(new_size) => {
              // Update size.
              shared_window.write().await.size = new_size;
              call_if_true!(
                DEBUG,
                shared_window.read().await.log_state("main_event_loop -> Resize")
              );
              perform_render(&shared_store, &shared_render, new_size).await?;
            }
            Continuation::Continue => {
              // Pass the event to the shared_render for further processing.
              let my_state = shared_store.read().await.get_state().await;
              let window_size = shared_window.read().await.size;
              shared_render
                .read()
                .await
                .handle_event(&input_event, &my_state, &shared_store, window_size)
                .await?
            }
          };
        }
        TWCommand::flush();
      }
    })
  }

  /// Dump the state of the terminal window to the log.
  pub fn log_state(&self, msg: &str) {
    log_no_err!(INFO, "{} -> {}", msg, self.to_string());
  }
}

pub async fn perform_render<S, A>(
  shared_store: &SharedStore<S, A>,
  shared_render: &SharedRender<S, A>,
  window_size: Size,
) -> CommonResult<()>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  throws!({
    let my_state = shared_store.read().await.get_state().await;
    shared_render
      .read()
      .await
      .render(&my_state, &shared_store, window_size)
      .await?
      .flush();
  });
}

struct MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send + 'static,
  A: Display + Default + Clone + Sync + Send + 'static,
{
  shared_render: SharedRender<S, A>,
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
      .shared_render
      .read()
      .await
      .render(&state, &self.shared_store, window_size)
      .await;
    match render_result {
      Err(error) => {
        TWCommand::flush();
        call_if_true!(
          DEBUG,
          log_no_err!(ERROR, "MySubscriber::run draw error: {}", error)
        );
      }
      Ok(mut command_queue) => {
        command_queue.flush();
        call_if_true!(
          DEBUG,
          log_no_err!(INFO, "MySubscriber::run draw: {}, {}", window_size, state)
        );
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
      shared_render: shared_draw.clone(),
      shared_store: shared_store.clone(),
      shared_window: shared_window.clone(),
    })
  }
}
