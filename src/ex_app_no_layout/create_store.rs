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
use r3bl_cmdr::*;
use r3bl_rs_utils::*;

// Create a new store and attach the reducer.
pub async fn create_store() -> Store<AppState, AppAction> {
  let mut store: Store<AppState, AppAction> = Store::default();
  store.add_reducer(AppReducer::new()).await;
  store
}

/// Action.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AppAction {
  Startup,
  AddPop(i32),
  SubPop(i32),
  Clear,
  Noop,
}

impl Default for AppAction {
  fn default() -> Self { AppAction::Noop }
}

impl Display for AppAction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

/// State.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AppState {
  pub stack: Vec<i32>,
  pub data: StateManageFocusData,
}

impl StateManageFocus for AppState {}

impl Default for AppState {
  fn default() -> Self {
    Self {
      stack: vec![0],
      data: StateManageFocusData::default(),
    }
  }
}

impl Display for AppState {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "State {{ stack: {:?}, data: {:?} }}", self.stack, self.data)
  }
}

/// Reducer.
#[derive(Default)]
pub struct AppReducer;

#[async_trait]
impl AsyncReducer<AppState, AppAction> for AppReducer {
  async fn run(&self, action: &AppAction, state: &AppState) -> AppState {
    let mut stack_copy = state.stack.clone();
    let _data_copy = state.data.clone();

    match action {
      AppAction::AddPop(arg) => {
        if stack_copy.is_empty() {
          stack_copy.push(*arg)
        } else {
          let top = stack_copy.pop().unwrap();
          let sum = top + arg;
          stack_copy.push(sum);
        }
      }

      AppAction::SubPop(arg) => {
        if stack_copy.is_empty() {
          stack_copy.push(*arg)
        } else {
          let top = stack_copy.pop().unwrap();
          let sum = top - arg;
          stack_copy.push(sum);
        }
      }

      AppAction::Clear => stack_copy = vec![],

      _ => {}
    }

    AppState {
      stack: stack_copy,
      data: _data_copy,
    }
  }
}
