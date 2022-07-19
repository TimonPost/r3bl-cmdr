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

use std::fmt::{Display, Formatter};

use r3bl_cmdr::StateManageFocus;

/// State.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct AppState {
  pub id_with_focus: String,
  pub stack: Vec<i32>,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      stack: vec![0],
      id_with_focus: "".into(),
    }
  }
}

impl StateManageFocus for AppState {
  fn get_focus_id(&self) -> String { self.id_with_focus.clone() }
  fn set_focus_id(&mut self, arg: String) { self.id_with_focus = arg }
}

impl Display for AppState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "State {{ stack: {:?}, id_with_focus: {:?} }}",
      self.stack, self.id_with_focus
    )
  }
}
