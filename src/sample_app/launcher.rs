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

pub async fn run_app() -> CommonResult<()> {
  throws!({
    // Create store.
    let mut store: Store<AppState, AppAction> = Store::default();

    // Attach reducer.
    store.add_reducer(AppReducer::new()).await;

    // Create an App (renders & responds to user input).
    let shared_render = App::new_shared();

    // Create a window.
    TerminalWindow::start_event_loop(store, shared_render).await?
  });
}
