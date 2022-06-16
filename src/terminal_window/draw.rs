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

#[async_trait]
pub trait Draw<S>
where
  S: Send + Sync,
{
  /// Given the state and input_event, render the output (via crossterm). To change the
  /// state, it is necessary to dispatch a redux action.
  async fn draw(&self, state: &S, input_event: &InputEvent) -> CommonResult<()>;

  /// https://doc.rust-lang.org/book/ch10-02-traits.html
  fn new() -> Box<dyn Draw<S>>
  where
    Self: Default + Sync + Send + 'static,
  {
    Box::new(Self::default())
  }
}
