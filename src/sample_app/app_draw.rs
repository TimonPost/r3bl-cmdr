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

use async_trait::async_trait;
use r3bl_cmdr::*;
use r3bl_rs_utils::*;
use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

/// Async trait object that implements the [Draw] trait.
#[derive(Default)]
pub struct AppDraw;

#[async_trait]
impl<S, A> Draw<S, A> for AppDraw
where
  S: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  async fn draw(&self, state: &S, _shared_store: &ShareStore<S, A>) -> CommonResult<()> {
    throws!({
      // TODO: remove debug
      println!("⛵ Draw -> draw: {}\r", state);
    });
  }

  async fn handle_event(
    &self,
    input_event: &InputEvent,
    state: &S,
    _shared_store: &ShareStore<S, A>,
  ) -> CommonResult<()> {
    throws!({
      // TODO: remove debug
      println!("🚀 handle_event: {} state: {}\r", input_event, state);
    });
  }
}
