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

use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use crossterm::event::*;
use r3bl_cmdr::*;
use r3bl_rs_utils::*;
use tokio::sync::RwLock;

use super::*;

/// Async trait object that implements the [TWApp] trait.
#[derive(Default)]
pub struct AppWithLayout {
  pub component_registry: ComponentRegistry<AppWithLayoutState, AppWithLayoutAction>,
  pub has_focus: HasFocus,
}

impl Debug for AppWithLayout {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("AppWithLayout")
      .field("component_registry", &self.component_registry)
      .field("state_manage_focus_data", &self.has_focus)
      .finish()
  }
}

// Constants for the ids.
const CONTAINER_ID: &str = "container";
const COL_1_ID: &str = "col_1";
const COL_2_ID: &str = "col_2";

#[async_trait]
impl TWApp<AppWithLayoutState, AppWithLayoutAction> for AppWithLayout {
  async fn app_handle_event(
    &mut self, input_event: &TWInputEvent, _state: &AppWithLayoutState,
    _shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>, _terminal_size: Size,
  ) -> CommonResult<EventPropagation> {
    throws_with_return!({
      // let mut event_consumed = false;

      // // Handle Left, Right to switch focus between columns.
      // if let TWInputEvent::NonDisplayableKeypress(key_event) = input_event {
      //   match key_event {
      //     KeyEvent {
      //       code: KeyCode::Left,
      //       modifiers: KeyModifiers::NONE,
      //     } => {
      //       event_consumed = true;
      //       self.switch_focus(KeyCode::Left);
      //       debug_log_has_focus(
      //         stringify!(AppWithLayout::app_handle_event).into(),
      //         &self.has_focus,
      //       );
      //     }
      //     KeyEvent {
      //       code: KeyCode::Right,
      //       modifiers: KeyModifiers::NONE,
      //     } => {
      //       event_consumed = true;
      //       self.switch_focus(KeyCode::Right);
      //       debug_log_has_focus(
      //         stringify!(AppWithLayout::app_handle_event).into(),
      //         &self.has_focus,
      //       );
      //     }
      //     _ => {}
      //   }
      // }

      // if event_consumed {
      //   return Ok(EventPropagation::ConsumedRerender);
      // }

      if let Continuation::Break = self.handle_left_right_input_to_switch_focus(input_event) {
        return Ok(EventPropagation::ConsumedRerender);
      }

      if let Some(_shared_component_has_focus) =
        self.component_registry.get_has_focus(&self.has_focus)
      {
        // FIXME: route event to component_registry[id w/ focus]
      };

      EventPropagation::Propagate
    });
  }

  async fn app_render(
    &mut self, state: &AppWithLayoutState,
    shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>, window_size: Size,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      self.create_components_populate_registry_init_focus().await;
      let mut tw_surface = TWSurface {
        stylesheet: self.create_stylesheet()?,
        ..TWSurface::default()
      };
      tw_surface.surface_start(TWSurfaceProps {
        pos: (0, 0).into(),
        size: window_size,
      })?;
      self
        .create_main_container(&mut tw_surface, state, shared_store)
        .await?;
      tw_surface.surface_end()?;
      tw_surface.render_buffer
    });
  }
}

impl AppWithLayout {
  fn handle_left_right_input_to_switch_focus(
    &mut self, input_event: &TWInputEvent,
  ) -> Continuation {
    let mut event_consumed = false;

    // Handle Left, Right to switch focus between columns.
    if let TWInputEvent::NonDisplayableKeypress(key_event) = input_event {
      match key_event {
        KeyEvent {
          code: KeyCode::Left,
          modifiers: KeyModifiers::NONE,
        } => {
          event_consumed = true;
          self.switch_focus(KeyCode::Left);
          debug_log_has_focus(
            stringify!(AppWithLayout::app_handle_event).into(),
            &self.has_focus,
          );
        }
        KeyEvent {
          code: KeyCode::Right,
          modifiers: KeyModifiers::NONE,
        } => {
          event_consumed = true;
          self.switch_focus(KeyCode::Right);
          debug_log_has_focus(
            stringify!(AppWithLayout::app_handle_event).into(),
            &self.has_focus,
          );
        }
        _ => {}
      }
    }

    if event_consumed {
      Continuation::Break
    } else {
      Continuation::Continue
    }
  }

  fn switch_focus(&mut self, code: KeyCode) {
    if let Some(_id) = self.has_focus.get_id() {
      if code == KeyCode::Left {
        self.has_focus.set_id(COL_1_ID)
      } else {
        self.has_focus.set_id(COL_2_ID)
      }
    } else {
      log_no_err!(ERROR, "No focus id has been set, and it should be set!");
    }
  }

  async fn create_components_populate_registry_init_focus(&mut self) {
    let _component = ColumnRenderComponent::default();
    let shared_component_r1 = Arc::new(RwLock::new(_component));
    let shared_component_r2 = shared_component_r1.clone();

    // Construct "col_1".
    if self.component_registry.id_does_not_exist(COL_1_ID) {
      self.component_registry.put(COL_1_ID, shared_component_r1);
    }

    // Construct "col_2".
    if self.component_registry.id_does_not_exist(COL_2_ID) {
      self.component_registry.put(COL_2_ID, shared_component_r2);
    }

    // Init has focus.
    if self.has_focus.get_id().is_none() {
      self.has_focus.set_id(COL_1_ID);
    }
  }

  /// Main container CONTAINER_ID.
  async fn create_main_container<'a>(
    &mut self, tw_surface: &mut TWSurface, state: &'a AppWithLayoutState,
    shared_store: &'a SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      tw_surface.box_start(TWBoxProps {
        id: CONTAINER_ID.into(),
        dir: Direction::Horizontal,
        req_size: (100, 100).try_into()?,
        ..Default::default()
      })?;
      self
        .create_left_col(tw_surface, state, shared_store)
        .await?;
      self
        .create_right_col(tw_surface, state, shared_store)
        .await?;
      tw_surface.box_end()?;
    });
  }

  /// Left column COL_1_ID.
  async fn create_left_col<'a>(
    &mut self, tw_surface: &mut TWSurface, _state: &'a AppWithLayoutState,
    _shared_store: &'a SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      tw_surface.box_start(TWBoxProps {
        styles: tw_surface.stylesheet.find_styles_by_ids(vec!["style1"]),
        id: COL_1_ID.into(),
        dir: Direction::Vertical,
        req_size: (50, 100).try_into()?,
      })?;

      // UGLY: consider adding a macro for this block
      if let Some(shared_component) = self.component_registry.get(COL_1_ID) {
        let current_box = tw_surface.current_box()?;
        let queue = shared_component
          .write()
          .await
          .render(current_box, _state, _shared_store)
          .await?;
        tw_surface.render_buffer += queue;
      }

      tw_surface.box_end()?;
    });
  }

  /// Right column COL_2_ID.
  async fn create_right_col(
    &mut self, tw_surface: &mut TWSurface, _state: &AppWithLayoutState,
    _shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<()> {
    throws!({
      tw_surface.box_start(TWBoxProps {
        styles: tw_surface.stylesheet.find_styles_by_ids(vec!["style2"]),
        id: COL_2_ID.to_string(),
        dir: Direction::Vertical,
        req_size: (50, 100).try_into()?,
      })?;

      // UGLY: consider adding a macro for this block
      if let Some(shared_component) = self.component_registry.get(COL_2_ID) {
        let current_box = tw_surface.current_box()?;
        let queue = shared_component
          .write()
          .await
          .render(current_box, _state, _shared_store)
          .await?;
        tw_surface.render_buffer += queue;
      }

      tw_surface.box_end()?;
    });
  }

  fn create_stylesheet(&mut self) -> CommonResult<Stylesheet> {
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
}
