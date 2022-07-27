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

use async_trait::async_trait;
use r3bl_cmdr::*;
use r3bl_rs_utils::*;

use super::*;

pub struct ColumnRenderComponent<'a> {
  pub lolcat: &'a mut Lolcat,
}

#[async_trait]
impl<'a> RenderComponent<AppState, AppAction> for ColumnRenderComponent<'a> {
  async fn render_component(
    &mut self, current_box: &TWBox, _state: &AppState, _shared_store: &SharedStore<AppState, AppAction>,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      let first_line = "col 1 - Hello".to_string();
      let second_line = "col 1 - World".to_string();

      let box_origin_pos = current_box.origin_pos; // Adjusted for style margin (if any).
      let box_bounding_size = current_box.bounding_size; // Adjusted for style margin (if any).
      let mut content_pos = Position { col: 0, row: 0 };

      // First line.
      let move_cursor_to_first_line_cmd = TWCommand::MoveCursorPositionRelTo(box_origin_pos, content_pos);
      let style_cmd = TWCommand::ApplyColors(current_box.get_computed_style());
      let first_line = box_bounding_size.truncate_at_cols(first_line);
      let first_line = colorize_using_lolcat! {
        &mut self.lolcat,
        "{}",
        first_line
      };

      let print_first_line_cmd = TWCommand::PrintWithAttributes(first_line, current_box.get_computed_style());

      // Second line.
      content_pos.add_row_with_bounds(1, box_bounding_size);
      let move_cursor_to_second_line_cmd = TWCommand::MoveCursorPositionRelTo(box_origin_pos, content_pos);
      let second_line = colorize_using_lolcat! {
        &mut self.lolcat,
        "{}",
        box_bounding_size.truncate_at_cols(second_line)
      };
      let print_second_line_cmd = TWCommand::PrintWithAttributes(second_line, current_box.get_computed_style());

      // Reset.
      let reset_color_cmd = TWCommand::ResetColor;

      let queue = tw_queue! {
        move_cursor_to_first_line_cmd,
        style_cmd,
        print_first_line_cmd,
        move_cursor_to_second_line_cmd,
        print_second_line_cmd,
        reset_color_cmd
      };

      call_if_true!(DEBUG, {
        log_no_err! {
          INFO,
          "🦜 ColumnComponent::render -> current_box: {:?},
            \n - box_origin_pos: {:?},
            \n - box_bounding_size: {:?}, 
            \n - content_pos: {:?},
            \n - queue: {:?}",
          current_box,
          box_origin_pos,
          box_bounding_size,
          content_pos,
          queue
        };
      });

      queue
    });
  }
}
