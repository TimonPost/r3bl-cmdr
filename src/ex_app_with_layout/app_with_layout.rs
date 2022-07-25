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
 *   Unless required by applicable law or agreed &to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use async_trait::async_trait;
use crossterm::event::*;
use r3bl_cmdr::*;
use r3bl_rs_utils::*;

use super::*;

/// Async trait object that implements the [Render] trait.
#[derive(Default, Debug, Clone, Copy)]
pub struct AppWithLayout {
  pub lolcat: Lolcat,
}

#[async_trait]
impl TWApp<AppState, AppAction> for AppWithLayout {
  async fn render(
    &mut self, state: &AppState, shared_store: &SharedStore<AppState, AppAction>, window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      let mut tw_surface = TWSurface {
        stylesheet: create_stylesheet()?,
        ..TWSurface::default()
      };
      tw_surface.surface_start(
        TWAreaPropsBuilder::new()
          .set_pos((0, 0).into())
          .set_size(window_size)
          .build(),
      )?;
      create_main_container(&mut tw_surface, &mut self.lolcat, state, shared_store).await?;
      tw_surface.surface_end()?;
      tw_surface.render_buffer
    });
  }

  async fn handle_event(
    &self, input_event: &TWInputEvent, _state: &AppState, shared_store: &SharedStore<AppState, AppAction>,
    _terminal_size: Size,
  ) -> CommonResult<()> {
    throws!({
      if let TWInputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            debug_log(AppAction::AddPop(1));
          }
          '-' => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            debug_log(AppAction::SubPop(1));
          }
          _ => {}
        }
      }
      if let TWInputEvent::NonDisplayableKeypress(key_event) = input_event {
        match key_event {
          KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            debug_log(AppAction::AddPop(1));
          }
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            debug_log(AppAction::SubPop(1));
          }
          _ => {}
        }
      }
    });
  }
}

fn debug_log(action: AppAction) {
  call_if_true!(
    DEBUG,
    log_no_err!(INFO, "⛵ AppWithLayout::handle_event -> dispatch_spawn: {}", action)
  );
}

/// Main container "container".
async fn create_main_container<'a>(
  tw_surface: &mut TWSurface, lolcat: &'a mut Lolcat, state: &'a AppState,
  shared_store: &'a SharedStore<AppState, AppAction>,
) -> CommonResult<()> {
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_id("container".to_string())
        .set_dir(Direction::Horizontal)
        .set_req_size((100, 100).try_into()?)
        .build(),
    )?;
    create_left_col(tw_surface, lolcat, state, shared_store).await?;
    create_right_col(tw_surface, lolcat, state, shared_store);
    tw_surface.box_end()?;
  });
}

/// Left column "col_1".
async fn create_left_col<'a>(
  tw_surface: &mut TWSurface, lolcat: &'a mut Lolcat, state: &'a AppState,
  shared_store: &'a SharedStore<AppState, AppAction>,
) -> CommonResult<()> {
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_surface.stylesheet.find_styles_by_ids(vec!["style1"]))
        .set_id("col_1".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;
    let mut column_component = ColumnComponent { lolcat };

    let current_box = tw_surface.current_box()?;

    let tw_queue: TWCommandQueue = column_component.render(current_box, state, shared_store).await?;

    tw_surface.render_buffer += tw_queue;
    tw_surface.box_end()?;
  });
}

struct ColumnComponent<'a> {
  lolcat: &'a mut Lolcat,
}

#[async_trait]
impl<'a> RenderComponent<AppState, AppAction> for ColumnComponent<'a> {
  async fn render(
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

/// Right column "col_2".
fn create_right_col(
  tw_surface: &mut TWSurface, lolcat: &mut Lolcat, state: &AppState, shared_store: &SharedStore<AppState, AppAction>,
) -> CommonResult<()> {
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_surface.stylesheet.find_styles_by_ids(vec!["style2"]))
        .set_id("col_2".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;

    let first_line = "col 2 - Hello".to_string();
    colorize_using_lolcat! {
      lolcat,
      "{}",
      first_line
    };

    let second_line = "col 2 - Hello".to_string();
    colorize_using_lolcat! {
      lolcat,
      "{}",
      second_line
    };

    tw_surface.print_inside_box(vec![first_line.as_str()])?;
    tw_surface.print_inside_box(vec![second_line.as_str()])?;
    tw_surface.box_end()?;
  });
}

fn create_stylesheet() -> CommonResult<Stylesheet> {
  throws_with_return!({
    let mut stylesheet = Stylesheet::new();

    stylesheet.add_styles(vec![
      style! {
        id: style1
        margin: 1
        color_fg: Color::Rgb { r: 51, g: 255, b: 255 } /* Turquoise. */
        color_bg: Color::Rgb { r: 252, g: 157, b: 248 } /* Pink. */
      },
      style! {
        id: style2
        margin: 1
        color_fg: Color::White
        color_bg: Color::Magenta
      },
    ])?;

    stylesheet
  })
}
