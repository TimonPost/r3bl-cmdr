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

use std::collections::HashMap;

use r3bl_rs_utils::Position;

pub type CursorPositionMap = HashMap<String, Option<Position>>;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct StateManageFocusData {
  pub cursor_position_map: CursorPositionMap,
  pub id_with_focus: String,
}

/// There are certain fields that need to be in each state struct to represent global information
/// about keyboard focus. This trait exposes them as methods of that struct.
///
/// 1. An `id` [String] is used to store which [crate::TWBox] id currently holds keyboard focus.
///    This is global.
/// 2. Each `id` may have a [Position] associated with it, which is used to draw the "cursor" (the
///    meaning of which depends on the specific [crate::Component] impl). This cursor is
///    scoped to each `id` so it isn't strictly a single global value (like `id` itself). Here are
///    examples of what a "cursor" might mean for various [crate::Component]s:
///    - for an editor, it will be the insertion point where text is added / removed
///    - for a text viewer, it will be the cursor position which can be moved around
pub trait StateManageFocus {
  /// Set the id of the [crate::TWBox] that has keyboard focus.
  fn get_focus_id(&self, data: &StateManageFocusData) -> String { data.id_with_focus.clone() }

  /// Get the id of the [crate::TWBox] that has keyboard focus.
  fn set_focus_id(&mut self, data: &mut StateManageFocusData, id: &str) { data.id_with_focus = id.into(); }

  /// For a given [crate::TWBox] id, set the position of the cursor inside of it.
  fn set_cursor_position_for_id(
    &mut self, data: &mut StateManageFocusData, id: &str, maybe_position: Option<Position>,
  ) {
    let map = &mut data.cursor_position_map;
    map.insert(id.into(), maybe_position);
  }

  /// For a given [crate::TWBox] id, get the position of the cursor inside of it.
  fn get_cursor_position_for_id(&self, data: &StateManageFocusData, id: &str) -> Option<Position> {
    let map = &data.cursor_position_map;
    if map.contains_key(id) {
      *map.get(id).unwrap()
    } else {
      None
    }
  }
}
