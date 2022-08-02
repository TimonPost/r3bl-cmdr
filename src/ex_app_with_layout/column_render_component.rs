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
use r3bl_rs_utils::*;

use super::*;

#[derive(Debug, Clone, Default)]
pub struct ColumnRenderComponent {
  pub lolcat: Lolcat,
}

#[async_trait]
impl Component<AppWithLayoutState, AppWithLayoutAction> for ColumnRenderComponent {
  /// Handle following input events (and consume them):
  /// - Up, `+`   : fire `AddPop(1)`
  /// - Down, `-` : fire `SubPop(1)`
  async fn handle_event(
    &mut self, input_event: &TWInputEvent, _state: &AppWithLayoutState,
    shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<EventPropagation> {
    throws_with_return!({
      let mut event_consumed = false;

      if let TWInputEvent::DisplayableKeypress(typed_char) = input_event {
        match typed_char {
          '+' => {
            spawn_and_consume_event!(event_consumed, shared_store, AppWithLayoutAction::AddPop(1));
            debug_log_action(
              stringify!(ColumnRenderComponent::handle_event).into(),
              AppWithLayoutAction::AddPop(1),
            );
          }
          '-' => {
            spawn_and_consume_event!(event_consumed, shared_store, AppWithLayoutAction::SubPop(1));
            debug_log_action(
              stringify!(ColumnRenderComponent::handle_event).into(),
              AppWithLayoutAction::SubPop(1),
            );
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
            spawn_and_consume_event!(event_consumed, shared_store, AppWithLayoutAction::AddPop(1));
            debug_log_action(
              stringify!(ColumnRenderComponent::handle_event).into(),
              AppWithLayoutAction::AddPop(1),
            );
          }
          KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
          } => {
            spawn_and_consume_event!(event_consumed, shared_store, AppWithLayoutAction::SubPop(1));
            debug_log_action(
              stringify!(ColumnRenderComponent::handle_event).into(),
              AppWithLayoutAction::SubPop(1),
            );
          }
          _ => {}
        }
      }

      if event_consumed {
        EventPropagation::Consumed
      } else {
        EventPropagation::Propagate
      }
    });
  }

  async fn render(
    &mut self, has_focus: &HasFocus, current_box: &TWBox, _state: &AppWithLayoutState,
    _shared_store: &SharedStore<AppWithLayoutState, AppWithLayoutAction>,
  ) -> CommonResult<TWCommandQueue> {
    throws_with_return!({
      // Fixed strings.
      let line_1 = format!("{} - Hello", current_box.id);
      let line_2 = format!("{} - World", current_box.id);

      // Setup intermediate vars.
      let box_origin_pos = current_box.origin_pos; // Adjusted for style margin (if any).
      let box_bounding_size = current_box.bounding_size; // Adjusted for style margin (if any).
      let mut content_cursor_pos = Position { col: 0, row: 0 };
      let mut queue: TWCommandQueue = tw_command_queue!();

      // Line 1.
      tw_command_queue! {
        queue push
        TWCommand::MoveCursorPositionRelTo(box_origin_pos, content_cursor_pos),
        TWCommand::ApplyColors(current_box.get_computed_style()),
        TWCommand::PrintWithAttributes(
          colorize_using_lolcat! {
            &mut self.lolcat,
            "{}",
            line_1.unicode_string().truncate_to_fit_size(box_bounding_size)
          },
          current_box.get_computed_style(),
        )
      };

      // Line 2.
      tw_command_queue! {
        queue push
        TWCommand::MoveCursorPositionRelTo(
          box_origin_pos,
          content_cursor_pos.add_row_with_bounds(1, box_bounding_size)
        ),
        TWCommand::PrintWithAttributes(
          colorize_using_lolcat! {
            &mut self.lolcat,
            "{}",
            line_2.unicode_string().truncate_to_fit_size(box_bounding_size)
          },
          current_box.get_computed_style(),
        ),
        TWCommand::ResetColor
      };

      // Paint is_focused.
      if has_focus.does_current_box_have_focus(current_box) {
        tw_command_queue! {
          queue push
          TWCommand::MoveCursorPositionRelTo(
            box_origin_pos,
            content_cursor_pos.add_row_with_bounds(1, box_bounding_size)
          ),
          TWCommand::PrintWithAttributes("👀".into(), None)
        };
      }

      call_if_true!(DEBUG, {
        log_no_err! {
          INFO,
          "\
🦜 ColumnComponent::render ->
  - current_box: {:?},
  - box_origin_pos: {:?},
  - box_bounding_size: {:?},
  - content_pos: {:?},
  - queue: {:?}",
          current_box,
          box_origin_pos,
          box_bounding_size,
          content_cursor_pos,
          queue
        };
      });

      // Return the command queue.
      queue
    });
  }
}
