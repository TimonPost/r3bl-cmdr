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
use r3bl_rs_utils::*;
use std::sync::Arc;
use tokio::sync::RwLock;

// Type aliases.
pub type ShareStore<S, A> = Arc<RwLock<Store<S, A>>>;
pub type SafeDraw<S, A> = dyn Draw<S, A> + Send + Sync;
pub type ShareDraw<S, A> = Arc<RwLock<SafeDraw<S, A>>>;
