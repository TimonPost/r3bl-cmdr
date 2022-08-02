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

use std::borrow::Cow;

use r3bl_rs_utils::*;

// Attach sources.
pub mod ex_app_no_layout;
pub mod ex_app_with_layout;
pub mod ex_editor;
pub mod ex_lolcat;

// Use things from sources.
pub use ex_app_no_layout::*;
pub use ex_app_with_layout::*;
pub use ex_editor::*;
pub use ex_lolcat::*;
use reedline::*;

const HELP_MSG: &str = "\
Type a number to run corresponding example:
  1. App with no layout ❌
  2. App with layout ✅
  3. lolcat 🦜
  4. Text editor 📜
or type Ctrl+C / Ctrl+D / 'x' to exit";

#[tokio::main]
async fn main() -> CommonResult<()> {
  throws!({
    println!("{}", HELP_MSG);
    let selection = get_user_selection();
    run_ex_for_user_selection(selection).await?;
  })
}

async fn run_ex_for_user_selection(selection: Cow<'_, str>) -> CommonResult<()> {
  throws!({
    if !selection.is_empty() {
      match selection.as_ref() {
        "1" => throws!(ex_app_no_layout::run_app().await?),
        "2" => throws!(ex_app_with_layout::run_app().await?),
        "3" => throws!(ex_lolcat::run_app().await?),
        "4" => todo!("TODO: implement editor ex!"),
        _ => unimplemented!(),
      }
    }
  })
}

fn get_user_selection<'a>() -> Cow<'a, str> {
  let mut line_editor = Reedline::create();
  let prompt = DefaultPrompt::default();
  let mut selection: Cow<str> = Cow::from("");

  loop {
    let maybe_signal = &line_editor.read_line(&prompt);
    if let Ok(Signal::Success(user_input_str)) = maybe_signal {
      match user_input_str.as_str() {
        code @ ("1" | "2" | "3") => {
          selection.to_mut().push_str(code);
          break;
        }
        "x" => break,
        _ => println!("Unknown command: {}", user_input_str),
      }
    } else if let Ok(Signal::CtrlC) | Ok(Signal::CtrlD) = maybe_signal {
      break;
    }
  }

  selection
}
