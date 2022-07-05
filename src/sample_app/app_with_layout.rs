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

use crate::*;
use async_trait::async_trait;
use crossterm::{event::*, style::Color};

/// Async trait object that implements the [Draw] trait.
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
    &mut self,
    state: &AppState,
    _shared_store: &SharedStore<AppState, AppAction>,
    window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      let mut tw_surface = TWSurface {
        stylesheet: create_stylesheet()?,
        ..TWSurface::default()
      };
      tw_surface.surface_start(TWAreaPropsBuilder::new().set_pos((0, 0).into()).set_size(window_size).build())?;
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
    &self,
    input_event: &InputEvent,
    _state: &AppState,
    shared_store: &SharedStore<AppState, AppAction>,
    _terminal_size: Size,
  ) -> CommonResult<()> {
    throws!({
      call_if_true!(
        DEBUG,
        log_no_err!(INFO, "⛵ AppWithLayout::handle_event -> input_event: {}", input_event)
      );

      if let InputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_dispatch_action!(shared_store, AppAction::AddPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppWithLayout::handle_event -> + -> dispatch_spawn: {}",
                AppAction::AddPop(1)
              )
            );
          }
          '-' => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppWithLayout::handle_event -> - -> dispatch_spawn: {}",
                AppAction::SubPop(1)
              )
            );
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
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppWithLayout::handle_event -> Up -> dispatch_spawn: {}",
                AppAction::AddPop(1)
              )
            );
          }
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_dispatch_action!(shared_store, AppAction::SubPop(1));
            call_if_true!(
              DEBUG,
              log_no_err!(
                INFO,
                "⛵ AppWithLayout::handle_event -> Down -> dispatch_spawn: {}",
                AppAction::SubPop(1)
              )
            );
          }
          _ => {}
        }
      }
    });
  }
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
fn create_left_col(tw_surface: &mut TWSurface, render_props: &RenderProps) -> CommonResult<()> {
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
fn create_right_col(tw_surface: &mut TWSurface, render_props: &RenderProps) -> CommonResult<()> {
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

/// Create a stylesheet containing styles.
fn create_stylesheet() -> CommonResult<Stylesheet> {
  let mut stylesheet = Stylesheet::new();
  stylesheet.add_styles(vec![create_style1(), create_style2()])?;
  Ok(stylesheet)
}

fn create_style1() -> Style {
  let turquoise = Color::Rgb { r: 51, g: 255, b: 255 };
  let pink = Color::Rgb { r: 252, g: 157, b: 248 };
  StyleBuilder::new()
    .set_id("style1".to_string())
    .set_color_fg(Some(turquoise.into()))
    .set_color_bg(Some(pink.into()))
    .set_margin(Some(1))
    .build()
}

fn create_style2() -> Style {
  let white = Color::White;
  let magenta = Color::Magenta;
  StyleBuilder::new()
    .set_id("style2".to_string())
    .set_color_fg(Some(white.into()))
    .set_color_bg(Some(magenta.into()))
    .set_margin(Some(1))
    .build()
}
