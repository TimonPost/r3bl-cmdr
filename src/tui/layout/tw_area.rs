/*
 *   Copyright (c) 2022 Nazmul Idris
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

use crate::layout::*;
use crate::*;
use r3bl_rs_utils::*;

/// Represents a rectangular area of the terminal screen, and not necessarily the full
/// terminal screen.
#[derive(Clone, Debug, Default)]
pub struct TWArea {
  pub origin_pos: Position,
  pub box_size: Size,
  pub stack_of_boxes: Vec<TWBox>,
  pub stylesheet: Stylesheet,
  pub render_buffer: TWCommandQueue,
}

impl LayoutManagement for TWArea {
  fn area_start(&mut self, TWAreaProps { pos, size }: TWAreaProps) -> CommonResult<()> {
    throws!({
      // Expect stack to be empty!
      if !self.stack_of_boxes.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedAreaStart,
          LayoutError::format_msg_with_stack_len(&self.stack_of_boxes, "Stack of boxes should be empty"),
        )?
      }
      self.origin_pos = pos;
      self.box_size = size;
    });
  }

  fn area_end(&mut self) -> CommonResult<()> {
    throws!({
      // Expect stack to be empty!
      if !self.stack_of_boxes.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedAreaEnd,
          LayoutError::format_msg_with_stack_len(&self.stack_of_boxes, "Stack of boxes should be empty"),
        )?
      }
    });
  }

  fn box_start(&mut self, tw_box_props: TWBoxProps) -> CommonResult<()> {
    throws!({
      match self.stack_of_boxes.is_empty() {
        true => self.add_root_box(tw_box_props),
        false => self.add_box(tw_box_props),
      }?
    });
  }

  fn box_end(&mut self) -> CommonResult<()> {
    throws!({
      // Expect stack not to be empty!
      if self.stack_of_boxes.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedBoxEnd,
          LayoutError::format_msg_with_stack_len(&self.stack_of_boxes, "Stack of boxes should not be empty"),
        )?
      }
      self.stack_of_boxes.pop();
    });
  }

  fn print_inside_box(&mut self, text_vec: Vec<&str>) -> CommonResult<()> {
    throws!({
      for text in text_vec {
        // Get the line of text.
        let content_rows: UnitType = 1;

        // TODO: Use `convert_to_base_unit!(text.len());` w/ graphemes & text wrapping.
        let _content_cols = convert_to_base_unit!(text.len());
        let content_cols: UnitType = 0;

        // Update the `content_cursor_pos` (will be initialized for `self.current_box()` if it
        // doesn't exist yet).
        let content_size = (content_cols, content_rows).into();
        let content_relative_pos = self.calc_where_to_insert_new_content_in_box(content_size)?;

        // Get the current box & its style.
        let current_box = self.current_box()?;
        let box_origin_pos = current_box.origin_pos; // Adjusted for style margin.
        let _box_bound_size = current_box.bounds_size; // Adjusted for style margin.

        // TODO: Use `_box_bound_size` and `_content_col` to wrap or clip text.

        // Take `box_origin_pos` into account when calculating the `new_absolute_pos`.
        let new_absolute_pos = box_origin_pos + content_relative_pos;

        trace_log_no_err!(content_size);
        trace_log_no_err!(content_relative_pos);
        trace_log_no_err!(box_origin_pos);
        trace_log_no_err!(new_absolute_pos);

        // Queue a bunch of `TWCommand`s to paint the text.
        let move_to_cmd = TWCommand::MoveCursorPosition(new_absolute_pos.into());
        let style_cmd = TWCommand::ApplyColors(current_box.get_computed_style());
        let print_cmd = TWCommand::PrintWithAttributes(text.to_string(), current_box.get_computed_style());

        self.render_buffer += tw_queue!(move_to_cmd, style_cmd, print_cmd);
      }
    });
  }
}

impl PerformPositioningAndSizing for TWArea {
  /// 🌳 Root: Handle first box to add to stack of boxes, explicitly sized & positioned.
  fn add_root_box(
    &mut self,
    TWBoxProps {
      id,
      dir,
      req_size: RequestedSizePercent {
        width: width_pc,
        height: height_pc,
      },
      styles,
    }: TWBoxProps,
  ) -> CommonResult<()> {
    throws!({
      self.stack_of_boxes.push(TWBox::make_root_box(
        id,
        self.box_size,
        self.origin_pos,
        width_pc,
        height_pc,
        dir,
        Stylesheet::compute(styles),
      ));
    });
  }

  /// 🍀 Non-root: Handle non-root box to add to stack of boxes. [Position] and [Size] will be calculated.
  fn add_box(
    &mut self,
    TWBoxProps {
      id,
      dir,
      req_size: RequestedSizePercent {
        width: width_pc,
        height: height_pc,
      },
      styles,
    }: TWBoxProps,
  ) -> CommonResult<()> {
    throws!({
      let current_box = self.current_box()?;

      let container_bounds = current_box.bounds_size;

      let requested_size_allocation = Size::from((
        calc_percentage(width_pc, container_bounds.width),
        calc_percentage(height_pc, container_bounds.height),
      ));

      let old_position = unwrap_or_err! {
        current_box.box_cursor_pos,
        LayoutErrorType::BoxCursorPositionUndefined
      };

      self.calc_where_to_insert_new_box_in_tw_area(requested_size_allocation)?;

      self.stack_of_boxes.push(TWBox::make_box(
        id,
        dir,
        container_bounds,
        old_position,
        width_pc,
        height_pc,
        Stylesheet::compute(styles),
      ));
    });
  }

  /// Must be called *before* the new [TWBox] is added to the stack of boxes otherwise
  /// [LayoutErrorType::ErrorCalculatingNextLayoutPos] error is returned.
  ///
  /// This updates the `box_cursor_pos` of the current [TWBox].
  ///
  /// Returns the [Position] where the next [TWBox] can be added to the stack of boxes.
  fn calc_where_to_insert_new_box_in_tw_area(&mut self, allocated_size: Size) -> CommonResult<Position> {
    let current_box = self.current_box()?;
    let box_cursor_pos = current_box.box_cursor_pos;

    let box_cursor_pos = unwrap_or_err! {
      box_cursor_pos,
      LayoutErrorType::ErrorCalculatingNextBoxPos
    };

    let new_pos: Position = box_cursor_pos + allocated_size;

    // Adjust `new_pos` using Direction.
    let new_pos: Position = match current_box.dir {
      Direction::Vertical => new_pos * (0, 1).into(),
      Direction::Horizontal => new_pos * (1, 0).into(),
    };

    // Update the box_cursor_pos of the current layout.
    current_box.box_cursor_pos = new_pos.as_some();

    Ok(new_pos)
  }

  /// Update the `content_cursor_pos` of the current [TWBox] and return the original [Position] that
  /// was there prior to this update.h
  fn calc_where_to_insert_new_content_in_box(&mut self, content_size: Size) -> CommonResult<Position> {
    throws_with_return!({
      // Get current content_cursor_pos or initialize it to (0, 0).
      let current_box = self.current_box()?;
      let current_pos = unwrap_option_or_compute_if_none! {
        current_box.content_cursor_pos,
        || (0, 0).into()
      };

      // Calculate new_pos based on content_size.
      let new_pos = current_pos + content_size;

      // Update the content_cursor_pos.
      current_box.content_cursor_pos = Some(new_pos);

      // Return current_pos.
      current_pos
    });
  }

  /// Get the last box on the stack (if none found then return Err).
  fn current_box(&mut self) -> CommonResult<&mut TWBox> {
    // Expect stack of boxes not to be empty!
    if self.stack_of_boxes.is_empty() {
      LayoutError::new_err(LayoutErrorType::StackOfBoxesShouldNotBeEmpty)?
    }
    Ok(self.stack_of_boxes.last_mut().unwrap())
  }
}
