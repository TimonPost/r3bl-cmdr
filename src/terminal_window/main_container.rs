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
  pub async fn start_event_loop<S, A>(store: Store<S, A>, shared_draw: ShareDraw<S, A>) -> CommonResult<()>
  where
    S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
    A: Display + Default + Clone + Sync + Send,
  {
    raw_mode!({
      // Initialize the terminal window struct.
      let mut window = TerminalWindow::try_to_create_instance()?;

      // Move the store into an Arc & RwLock.
      let shared_store: ShareStore<S, A> = Arc::new(RwLock::new(store));

      // Attach a subscriber to the store.
      shared_store
        .write()
        .await
        .add_subscriber(MySubscriber::new_box(&shared_draw, &shared_store))
        .await;

      call_if_true!(DEBUG, window.log_state("Startup"));

      // Main event loop.
      loop {
        if let Some(input_event) = window.stream.try_to_get_input_event().await {
          if let Continuation::Exit = base_handle_event(input_event, &mut window).await {
            break;
          }
          let my_state = shared_store.read().await.get_state().await;
          shared_draw
            .read()
            .await
            .handle_event(&input_event, &my_state, &shared_store)
            .await?;
        }
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
  shared_draw: ShareDraw<S, A>,
  shared_store: ShareStore<S, A>,
}

#[async_trait]
impl<S, A> AsyncSubscriber<S> for MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  async fn run(&self, state: S) {
    let draw_result = self.shared_draw.read().await.draw(&state, &self.shared_store).await;
    if let Err(e) = draw_result {
      log_no_err!(ERROR, "TerminalWindowSubscriber::run draw error: {}", e)
    }
  }
}

impl<S, A> MySubscriber<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  fn new_box(shared_draw: &ShareDraw<S, A>, shared_store: &ShareStore<S, A>) -> Box<Self> {
    Box::new(MySubscriber {
      shared_draw: shared_draw.clone(),
      shared_store: shared_store.clone(),
    })
  }
}
