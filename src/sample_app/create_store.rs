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
use r3bl_rs_utils::redux::AsyncReducer;
use std::fmt::{Display, Formatter};

/// Action: Default + Clone + Sync + Send.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Action {
  AddPop(i32),
  SubPop(i32),
  Clear,
  Noop,
}

impl Default for Action {
  fn default() -> Self {
    Action::Noop
  }
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// State: Display + Default + Clone + PartialEq + Debug + Hash + Sync + Send.
#[derive(Default, Clone, PartialEq, Debug, Hash)]
pub struct State {
  pub stack: Vec<i32>,
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "State {{ stack: {:?} }}", self.stack)
  }
}

/// Reducer.
#[derive(Default)]
pub struct Reducer;

#[async_trait]
impl AsyncReducer<State, Action> for Reducer {
  async fn run(&self, action: &Action, state: &State) -> State {
    let mut stack = state.stack.clone();

    match action {
      Action::AddPop(arg) => {
        if stack.is_empty() {
          stack.push(*arg)
        } else {
          let top = stack.pop().unwrap();
          let sum = top + arg;
          stack.push(sum);
        }
      }

      Action::SubPop(arg) => {
        if stack.is_empty() {
          stack.push(*arg)
        } else {
          let top = stack.pop().unwrap();
          let sum = top - arg;
          stack.push(sum);
        }
      }

      Action::Clear => stack = vec![],

      _ => {}
    }

    State { stack }
  }
}
