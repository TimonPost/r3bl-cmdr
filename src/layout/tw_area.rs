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

use crate::layout::*;
use crate::*;
use r3bl_rs_utils::*;

/// Represents a rectangular area of the terminal screen, and not necessarily the full
/// terminal screen.
#[derive(Clone, Debug, Default)]
pub struct TWArea {
  pub origin: Position,
  pub size: Size,
  pub stack: Vec<TWBox>,
  pub stylesheet: Stylesheet,
  pub render_buffer: TWCommandQueue,
}

impl LayoutManagement for TWArea {
  fn area_start(&mut self, TWAreaProps { pos, size }: TWAreaProps) -> CommonResult<()> {
    throws!({
      // Expect stack to be empty!
      if !self.stack.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedAreaStart,
          LayoutError::format_msg_with_stack_len(
            &self.stack,
            "Layout
            stack should be empty",
          ),
        )?
      }
      self.origin = pos;
      self.size = size;
    });
  }

  fn area_end(&mut self) -> CommonResult<()> {
    throws!({
      // Expect stack to be empty!
      if !self.stack.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedAreaEnd,
          LayoutError::format_msg_with_stack_len(&self.stack, "Layout stack should be empty"),
        )?
      }
    });
  }

  fn box_start(&mut self, layout_props: TWBoxProps) -> CommonResult<()> {
    throws!({
      match self.stack.is_empty() {
        true => self.add_root_box(layout_props),
        false => self.add_box(layout_props),
      }?
    });
  }

  fn box_end(&mut self) -> CommonResult<()> {
    throws!({
      // Expect stack not to be empty!
      if self.stack.is_empty() {
        LayoutError::new_err_with_msg(
          LayoutErrorType::MismatchedBoxEnd,
          LayoutError::format_msg_with_stack_len(&self.stack, "Layout stack should not be empty"),
        )?
      }
      self.stack.pop();
    });
  }

  fn print_inside_box(&mut self, text_vec: Vec<&str>) -> CommonResult<()> {
    throws!({
      for text in text_vec {
        // Get the line of text.
        let content_x = 0;
        let _content_x = text.len().try_into().unwrap_or(text.len() as UnitType);
        let content_y = 1;

        // Update the content_cursor_pos (will be initialized for `self.current_box()` if
        // it doesn't exist yet).
        let content_size = (content_x, content_y).into();
        let new_pos = self.calc_where_to_insert_new_content_in_box(content_size)?;

        // Queue a bunch of TWCommand to paint the text.
        let move_to = TWCommand::MoveCursorPosition(new_pos.into());
        // TODO: handle styling
        let print = TWCommand::Print(text.to_string());

        self.render_buffer += tw_queue!(move_to, print);
      }
    });
  }
}

impl PerformPositioningAndSizing for TWArea {
  /// 🌳 Root: Handle first layout to add to stack, explicitly sized & positioned.
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
      self.stack.push(TWBox::make_root_box(
        id.to_string(),
        self.size,
        self.origin,
        width_pc,
        height_pc,
        dir,
        Stylesheet::compute(styles),
      ));
    });
  }

  /// 🍀 Non-root: Handle layout to add to stack. [Position] and [Size] will be calculated.
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

      let container_bounds = unwrap_or_err! {
        current_box.bounds_size,
        LayoutErrorType::ContainerBoxBoundsUndefined
      };

      let requested_size_allocation = Size::from((
        calc_percentage(width_pc, container_bounds.width),
        calc_percentage(height_pc, container_bounds.height),
      ));

      let old_position = unwrap_or_err! {
        current_box.box_cursor_pos,
        LayoutErrorType::BoxCursorPositionUndefined
      };

      self.calc_where_to_insert_new_box_in_tw_area(requested_size_allocation)?;

      self.stack.push(TWBox::make_box(
        id.to_string(),
        dir,
        container_bounds,
        old_position,
        width_pc,
        height_pc,
        Stylesheet::compute(styles),
      ));
    });
  }

  /// Must be called *before* the new [TWBox] is added to the stack otherwise
  /// [LayoutErrorType::ErrorCalculatingNextLayoutPos] error is returned.
  ///
  /// This updates the `box_cursor_pos` of the current [TWBox].
  ///
  /// Returns the [Position] where the next [TWBox] can be added to the stack.
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

  /// This updates the `content_cursor_pos` of the current [TWBox].
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

      // Return new_pos
      new_pos
    });
  }

  /// Get the last layout on the stack (if none found then return Err).
  fn current_box(&mut self) -> CommonResult<&mut TWBox> {
    // Expect stack not to be empty!
    if self.stack.is_empty() {
      LayoutError::new_err(LayoutErrorType::StackShouldNotBeEmpty)?
    }
    Ok(self.stack.last_mut().unwrap())
  }
}
