use r3bl_rs_utils::{TWCommand, TWCommandQueue, Size};

/// Appends commands to the queue that display a 'quit' hint at the center, bottom.
pub fn append_quit_msg_center_bottom(queue: &mut TWCommandQueue, size: Size)  {
    let message: String = "Press Ctrl + q to exit!".into();

    [
       TWCommand::MoveCursorPositionAbs(((size.cols / 2) - message.chars().count() as u16 / 2, size.rows - 1).into()),
       TWCommand::PrintWithAttributes(message, None),
    ].iter().for_each(|cmd| {
       queue.push(cmd.clone());
    });
 }