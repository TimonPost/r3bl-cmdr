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

use crossterm::style::Color;
use r3bl_cmdr::layout::*;
use r3bl_rs_utils::*;

#[test]
fn test_simple_2_col_layout() -> CommonResult<()> {
  throws!({
    let mut tw_area = TWArea {
      stylesheet: create_stylesheet()?,
      ..TWArea::default()
    };
    tw_area.area_start(TWAreaPropsBuilder::new().set_pos((0, 0).into()).set_size((500, 500).into()).build())?;
    create_main_container(&mut tw_area)?;
    tw_area.area_end()?;
    println!("{}", tw_area.render_buffer);
  });
}

/// Main container "container".
fn create_main_container(tw_area: &mut TWArea) -> CommonResult<()> {
  throws!({
    tw_area.box_start(
      TWBoxPropsBuilder::new()
        .set_id("container".to_string())
        .set_dir(Direction::Horizontal)
        .set_req_size((100, 100).try_into()?)
        .build(),
    )?;
    make_container_assertions(tw_area)?;
    create_left_col(tw_area)?;
    create_right_col(tw_area)?;
    tw_area.box_end()?;
  });

  fn make_container_assertions(tw_area: &TWArea) -> CommonResult<()> {
    throws!({
      let layout_item = tw_area.stack_of_boxes.first().unwrap();
      assert_eq!(layout_item.id, "container");
      assert_eq!(layout_item.dir, Direction::Horizontal);
      assert_eq!(layout_item.origin_pos, (0, 0).into());
      assert_eq!(layout_item.bounds_size, (500, 500).into());
      assert_eq!(layout_item.req_size_percent, (100, 100).try_into()?);
      assert_eq!(layout_item.box_cursor_pos, Some((0, 0).into()));
      assert_eq!(layout_item.content_cursor_pos, None);
      assert_eq!(layout_item.get_computed_style(), None);
    });
  }
}

/// Left column "col_1".
fn create_left_col(tw_area: &mut TWArea) -> CommonResult<()> {
  throws!({
    tw_area.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_area.stylesheet.find_styles_by_ids(vec!["style1"]))
        .set_id("col_1".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;
    tw_area.print_inside_box(vec!["col 1 - Hello"])?;
    tw_area.print_inside_box(vec!["col 1 - World"])?;
    make_left_col_assertions(tw_area)?;
    tw_area.box_end()?;
  });

  fn make_left_col_assertions(tw_area: &TWArea) -> CommonResult<()> {
    throws!({
      let layout_item = tw_area.stack_of_boxes.last().unwrap();
      assert_eq!(layout_item.id, "col_1");
      assert_eq!(layout_item.dir, Direction::Vertical);
      assert_eq!(layout_item.origin_pos, (2, 2).into()); // Take margin into account.
      assert_eq!(layout_item.bounds_size, (246, 496).into()); // Take margin into account.
      assert_eq!(layout_item.req_size_percent, (50, 100).try_into()?);
      assert_eq!(layout_item.box_cursor_pos, None);
      assert_eq!(layout_item.content_cursor_pos, Some((0, 2).into()));
      assert_eq!(
        layout_item.get_computed_style(),
        Stylesheet::compute(tw_area.stylesheet.find_styles_by_ids(vec!["style1"]))
      );
    });
  }
}

/// Right column "col_2".
fn create_right_col(tw_area: &mut TWArea) -> CommonResult<()> {
  throws!({
    tw_area.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_area.stylesheet.find_styles_by_ids(vec!["style2"]))
        .set_id("col_2".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;
    tw_area.print_inside_box(vec!["col 2 - Hello"])?;
    tw_area.print_inside_box(vec!["col 2 - World"])?;
    make_right_col_assertions(tw_area)?;
    tw_area.box_end()?;
  });

  fn make_right_col_assertions(tw_area: &TWArea) -> CommonResult<()> {
    throws!({
      let current_box = tw_area.stack_of_boxes.last().unwrap();
      assert_eq!(current_box.id, "col_2");
      assert_eq!(current_box.dir, Direction::Vertical);
      assert_eq!(current_box.origin_pos, (252, 2).into()); // Take margin into account.
      assert_eq!(current_box.bounds_size, (246, 496).into()); // Take margin into account.
      assert_eq!(current_box.req_size_percent, (50, 100).try_into()?);
      assert_eq!(current_box.box_cursor_pos, None);
      assert_eq!(current_box.content_cursor_pos, Some((0, 2).into()));
      assert_eq!(
        current_box.get_computed_style(),
        Stylesheet::compute(tw_area.stylesheet.find_styles_by_ids(vec!["style2"]))
      );
    });
  }
}

/// Create a stylesheet containing styles.
fn create_stylesheet() -> CommonResult<Stylesheet> {
  let mut stylesheet = Stylesheet::new();
  stylesheet.add_styles(vec![create_style("style1"), create_style("style2")])?;
  Ok(stylesheet)
}

/// Create a style.
fn create_style(id: &str) -> Style {
  let black = Color::Rgb { r: 0, g: 0, b: 0 };
  StyleBuilder::new()
    .set_id(id.to_string())
    .set_color_bg(Some(black.into()))
    .set_color_fg(Some(black.into()))
    .set_dim(true)
    .set_bold(true)
    .set_margin(Some(2))
    .build()
}
