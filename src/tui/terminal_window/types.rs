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

use std::sync::Arc;

use r3bl_rs_utils::Size;
use tokio::sync::RwLock;

use crate::*;

// TWData.
pub type SharedWindow = Arc<RwLock<TWData>>;

// TWApp.
pub type SafeTWApp<S, A> = dyn TWApp<S, A> + Send + Sync;
pub type SharedTWApp<S, A> = Arc<RwLock<SafeTWApp<S, A>>>;

// Component.
pub type SafeComponent<S, A> = dyn Component<S, A> + Send + Sync;
pub type SharedComponent<S, A> = Arc<RwLock<SafeComponent<S, A>>>;

// Continuation enum.
#[non_exhaustive]
pub enum Continuation {
  Exit,
  Continue,
  ResizeAndContinue(Size),
}

// Event propagation enum.
#[non_exhaustive]
pub enum EventPropagation {
  Consumed,
  Propagate,
}

#[macro_export]
macro_rules! spawn_and_consume_event {
  ($bool: ident, $shared_store: ident, $action: expr) => {
    $bool = true;
    spawn_dispatch_action!($shared_store, $action);
  };
}
