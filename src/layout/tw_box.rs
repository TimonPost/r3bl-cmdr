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
use r3bl_rs_utils::Builder;

/// Direction of the layout of the box.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
  Horizontal,
  Vertical,
}

impl Default for Direction {
  fn default() -> Direction {
    Direction::Horizontal
  }
}

/// A box is a rectangle with a position and size. The direction of the box determines how
/// it's contained elements are positioned.
#[derive(Clone, Default, Builder)]
pub struct TWBox {
  pub id: String,
  pub dir: Direction,
  pub origin: Option<Position>,
  pub bounds_size: Option<Size>,
  pub req_size_percent: Option<RequestedSizePercent>,
  pub box_cursor_pos: Option<Position>,
  pub content_cursor_pos: Option<Position>,
  pub styles: Option<Style>,
}

impl TWBox {
  /// Explicitly set the position & size of our box.
  pub fn make_root_box(
    id: String,
    size: Size,
    origin: Position,
    width_pc: Percent,
    height_pc: Percent,
    dir: Direction,
    style: Option<Style>,
  ) -> TWBox {
    let bounds_size = Size::from((
      calc_percentage(width_pc, size.width),
      calc_percentage(height_pc, size.height),
    ));
    TWBoxBuilder::new()
      .set_id(id)
      .set_dir(dir)
      .set_origin(origin.as_some())
      .set_bounds_size(bounds_size.as_some())
      .set_req_size_percent(Some((width_pc, height_pc).into()))
      .set_box_cursor_pos(origin.as_some())
      .set_styles(style)
      .build()
  }

  /// Actual position and size for our box will be calculated based on provided hints.
  pub fn make_box(
    id: String,
    dir: Direction,
    container_bounds: Size,
    origin: Position,
    width_pc: Percent,
    height_pc: Percent,
    style: Option<Style>,
  ) -> Self {
    // Adjust `bounds_size` & `origin` based on the style's margin.
    let mut style_adjusted_origin = origin;
    let mut style_adjusted_bounds_size = Size::from((
      calc_percentage(width_pc, container_bounds.width),
      calc_percentage(height_pc, container_bounds.height),
    ));
    if let Some(ref style) = style {
      if let Some(margin) = style.margin {
        style_adjusted_origin += margin;
        style_adjusted_bounds_size -= margin * 2;
      };
    }

    TWBoxBuilder::new()
      .set_id(id)
      .set_dir(dir)
      .set_origin(style_adjusted_origin.as_some())
      .set_bounds_size(style_adjusted_bounds_size.as_some())
      .set_req_size_percent(Some((width_pc, height_pc).into()))
      .set_styles(style)
      .build()
  }
}

macro_rules! format_option {
  ($opt:expr) => {
    match ($opt) {
      Some(v) => v,
      None => &FormatMsg::None,
    }
  };
}

impl std::fmt::Debug for TWBox {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TWBox")
      .field("id", &self.id)
      .field("dir", &self.dir)
      .field("origin", format_option!(&self.origin))
      .field("bounds_size", format_option!(&self.bounds_size))
      .field("req_size_percent", format_option!(&self.req_size_percent))
      .field("box_cursor_pos", format_option!(&self.box_cursor_pos))
      .field("content_cursor_pos", format_option!(&self.content_cursor_pos))
      .field("styles", format_option!(&self.styles))
      .finish()
  }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
enum FormatMsg {
  None,
}
