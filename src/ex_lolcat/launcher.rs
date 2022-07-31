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

use std::io::{BufRead, BufReader};

use tokio::fs::File;

use crate::*;

pub async fn run_app() -> CommonResult<()> {
  let mut my_lolcat = Lolcat::new();

  println!("{:?}", my_lolcat);

  let file = File::open("Cargo.lock").await?;
  let file = file.into_std().await;

  tokio::task::spawn_blocking(move || {
    let buffer_reader = BufReader::new(file);
    for (index, line) in buffer_reader.lines().enumerate() {
      let line = line.unwrap();
      println!("{}. {}", index + 1, my_lolcat.format_str(&line));
    }
  });

  Ok(())
}
