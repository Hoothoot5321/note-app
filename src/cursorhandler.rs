use crate::{cursorcontroller::CursorController, LinesManager};
use crossterm::event::KeyCode;
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}
pub struct CursorManager {
    pub cursor_controller: CursorController,
}
impl CursorManager {
    pub fn new() -> CursorManager {
        CursorManager {
            cursor_controller: CursorController::new(),
        }
    }
    pub fn get_cursor(&self) -> Cursor {
        Cursor {
            x: self.cursor_controller.get_x(),
            y: self.cursor_controller.get_y(),
        }
    }
    pub fn take_input(&mut self, direction: KeyCode, lines: &LinesManager) {
        match direction {
            KeyCode::Left => self.cursor_controller.move_left(),

            KeyCode::Right => self.cursor_controller.move_right(
                lines
                    .lines_controller
                    .get_line_size(self.cursor_controller.get_y())
                    .try_into()
                    .unwrap(),
            ),
            KeyCode::Up => {
                if self.cursor_controller.get_y() > 2 {
                    let fisk = lines
                        .lines_controller
                        .get_line_sizei_s(self.cursor_controller.get_y() - 1);
                    match fisk {
                        Ok(s) => self.cursor_controller.check_x(s as i32),
                        Err(_e) => {}
                    }
                }
                self.cursor_controller.move_up();
            }
            KeyCode::Down => {
                if self.cursor_controller.get_y() < lines.get_size() - 1 {
                    let fisk = lines
                        .lines_controller
                        .get_line_sizei_s(self.cursor_controller.get_y() + 1);
                    match fisk {
                        Ok(s) => self.cursor_controller.check_x(s as i32),
                        Err(_e) => {}
                    }
                }
                self.cursor_controller.move_down(lines.get_size() as i32);
            }
            _ => {}
        }
    }
}
