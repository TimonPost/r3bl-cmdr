/*
 *   Copyright (c) 2022 R3BL LLC
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

use std::{fmt::{Debug, Display},
          hash::Hash,
          sync::Arc};

use async_trait::async_trait;
use r3bl_rs_utils::*;
use tokio::sync::RwLock;

use crate::*;

/// Async trait docs: https://doc.rust-lang.org/book/ch10-02-traits.html
#[async_trait]
pub trait TWApp<S, A>
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  /// Use the state to render the output (via crossterm). To change the state, dispatch an action.
  async fn render(
    &mut self, state: &S, shared_store: &SharedStore<S, A>, window_size: Size,
  ) -> CommonResult<TWCommandQueue>;

  /// Use the input_event to dispatch an action to the store if needed.
  async fn handle_event(
    &self, input_event: &TWInputEvent, state: &S, shared_store: &SharedStore<S, A>, window_size: Size,
  ) -> CommonResult<()>;

  /// Wrap a new instance in [Box].
  fn new_owned() -> Box<dyn TWApp<S, A>>
  where
    Self: Default + Sync + Send + 'static,
  {
    Box::new(Self::default())
  }

  /// Wrap a new instance in [std::sync::Arc] & [tokio::sync::RwLock].
  fn new_shared() -> SharedRender<S, A>
  where
    Self: Default + Sync + Send + 'static,
  {
    Arc::new(RwLock::new(Self::default()))
  }
}
