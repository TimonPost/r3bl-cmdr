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
use crossterm::event::*;

use crate::*;

/// Async trait object that implements the [Render] trait.
#[derive(Default, Debug, Clone, Copy)]
pub struct AppWithLayout {
  pub lolcat: Lolcat,
}

struct RenderProps<'a> {
  pub lolcat: &'a Lolcat,
  pub state: &'a AppState,
}

#[async_trait]
impl Render<AppState, AppAction> for AppWithLayout {
  async fn render(
    &mut self, state: &AppState, _shared_store: &SharedStore<AppState, AppAction>, window_size: Size,
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
      create_main_container(
        &mut tw_surface,
        &RenderProps {
          lolcat: &self.lolcat,
          state,
        },
      )?;
      tw_surface.surface_end()?;
      tw_surface.render_buffer
    });
  }

  async fn handle_event(
    &self, input_event: &InputEvent, _state: &AppState, shared_store: &SharedStore<AppState, AppAction>,
    _terminal_size: Size,
  ) -> CommonResult<()> {
    throws!({
      if let InputEvent::DisplayableKeypress(typed_char) = input_event {
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
      if let InputEvent::NonDisplayableKeypress(key_event) = input_event {
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
fn create_main_container(tw_surface: &mut TWSurface, render_props: &RenderProps) -> CommonResult<()> {
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_id("container".to_string())
        .set_dir(Direction::Horizontal)
        .set_req_size((100, 100).try_into()?)
        .build(),
    )?;
    create_left_col(tw_surface, render_props)?;
    create_right_col(tw_surface, render_props)?;
    tw_surface.box_end()?;
  });
}

/// Left column "col_1".
fn create_left_col(tw_surface: &mut TWSurface, _render_props: &RenderProps) -> CommonResult<()> {
  // TODO: use render_props.lolcat to colorize render_props.state
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_surface.stylesheet.find_styles_by_ids(vec!["style1"]))
        .set_id("col_1".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;
    tw_surface.print_inside_box(vec!["col 1 - Hello"])?;
    tw_surface.print_inside_box(vec!["col 1 - World"])?;
    tw_surface.box_end()?;
  });
}

/// Right column "col_2".
fn create_right_col(tw_surface: &mut TWSurface, _render_props: &RenderProps) -> CommonResult<()> {
  // TODO: use render_props.lolcat to colorize render_props.state
  throws!({
    tw_surface.box_start(
      TWBoxPropsBuilder::new()
        .set_styles(tw_surface.stylesheet.find_styles_by_ids(vec!["style2"]))
        .set_id("col_2".to_string())
        .set_dir(Direction::Vertical)
        .set_req_size((50, 100).try_into()?)
        .build(),
    )?;
    tw_surface.print_inside_box(vec!["col 2 - Hello"])?;
    tw_surface.print_inside_box(vec!["col 2 - World"])?;
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