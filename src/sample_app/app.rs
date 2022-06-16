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
use std::fmt::{Display, Formatter};

/// Representation of the application state data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppState {
  pub focused: String,
  pub msg1: String,
  pub msg2: String,
}

/// For [ToString].
impl Display for AppState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// Async trait object that implements the [Draw] trait.
#[derive(Default)]
pub struct AppStateDraw;

#[async_trait]
impl<S> Draw<S> for AppStateDraw
where
  S: Send + Sync,
  S: Display, // For [ToString].
{
  async fn draw(&self, state: &S, input_event: &InputEvent) -> CommonResult<()> {
    throws!({
      println!("{} {}\r", state, input_event);
    });
  }
}
