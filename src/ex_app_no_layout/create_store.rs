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

use async_trait::async_trait;
use r3bl_rs_utils::*;

// Create a new store and attach the reducer.
pub async fn create_store() -> Store<AppNoLayoutState, AppNoLayoutAction> {
  let mut store: Store<AppNoLayoutState, AppNoLayoutAction> = Store::default();
  store.add_reducer(AppReducer::new()).await;
  store
}

/// Action.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AppNoLayoutAction {
  Startup,
  AddPop(i32),
  SubPop(i32),
  Clear,
  Noop,
}

impl Default for AppNoLayoutAction {
  fn default() -> Self { AppNoLayoutAction::Noop }
}

impl Display for AppNoLayoutAction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

/// State.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AppNoLayoutState {
  pub stack: Vec<i32>,
}

impl Default for AppNoLayoutState {
  fn default() -> Self { Self { stack: vec![0] } }
}

impl Display for AppNoLayoutState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "State {{ stack: {:?} }}", self.stack)
  }
}

/// Reducer.
#[derive(Default)]
pub struct AppReducer;

#[async_trait]
impl AsyncReducer<AppNoLayoutState, AppNoLayoutAction> for AppReducer {
  async fn run(&self, action: &AppNoLayoutAction, state: &AppNoLayoutState) -> AppNoLayoutState {
    let mut stack_copy = state.stack.clone();

    match action {
      AppNoLayoutAction::AddPop(arg) => {
        if stack_copy.is_empty() {
          stack_copy.push(*arg)
        } else {
          let top = stack_copy.pop().unwrap();
          let sum = top + arg;
          stack_copy.push(sum);
        }
      }

      AppNoLayoutAction::SubPop(arg) => {
        if stack_copy.is_empty() {
          stack_copy.push(*arg)
        } else {
          let top = stack_copy.pop().unwrap();
          let sum = top - arg;
          stack_copy.push(sum);
        }
      }

      AppNoLayoutAction::Clear => stack_copy = vec![],

      _ => {}
    }

    AppNoLayoutState { stack: stack_copy }
  }
}
