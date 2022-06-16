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
use r3bl_rs_utils::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// https://doc.rust-lang.org/book/ch10-02-traits.html
#[async_trait]
pub trait Draw<S>
where
  S: Send + Sync,
{
  // TODO: replace state -> store
  /// Use the state to render the output (via crossterm). To change the state, dispatch an action.
  async fn draw(&self, state: &S) -> CommonResult<()>;

  // TODO: replace state -> store
  /// Use the input_event to dispatch an action to the store if needed.
  async fn handle_event(&self, input_event: &InputEvent, state: &S) -> CommonResult<()>;

  /// Wrap a new instance in [Box].
  fn new_owned() -> Box<dyn Draw<S>>
  where
    Self: Default + Sync + Send + 'static,
  {
    Box::new(Self::default())
  }

  /// Wrap a new instance in [std::sync::Arc] & [tokio::sync::RwLock].
  fn new_shared() -> ShareDraw<S>
  where
    Self: Default + Sync + Send + 'static,
  {
    Arc::new(RwLock::new(Self::default()))
  }
}

pub type SafeDraw<S> = dyn Draw<S> + Send + Sync;
pub type ShareDraw<S> = Arc<RwLock<SafeDraw<S>>>;

// TODO: decide if some useful methods should be added to ShareDraw

// pub trait Foo {
//   fn foo(&self);
// }
//
// impl<S> Foo for ShareDraw<S> {
//   fn foo(&self) {
//     todo!()
//   }
// }
