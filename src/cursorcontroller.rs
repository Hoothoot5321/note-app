pub struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
}
impl CursorController {
    pub fn new() -> CursorController {
        CursorController {
            cursor_x: 0,
            cursor_y: 2,
        }
    }
    pub fn move_up(&mut self) {
        if self.cursor_y > 2 {
            self.cursor_y -= 1;
        }
    }

    pub fn move_down(&mut self, min_size: i32) {
        if self.cursor_y < (min_size - 1) as usize {
            self.cursor_y += 1;
        }
    }
    pub fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }
    pub fn move_right(&mut self, max_size: i32) {
        if self.cursor_x < max_size as usize {
            self.cursor_x += 1;
        }
    }
    pub fn set_x(&mut self, x: usize) {
        self.cursor_x = x;
    }
    pub fn get_x(&self) -> usize {
        self.cursor_x
    }
    pub fn check_x(&mut self, width: i32) {
        if self.cursor_x > width.try_into().unwrap() {
            self.set_x(width.try_into().unwrap())
        }
    }
    pub fn get_y(&self) -> usize {
        self.cursor_y
    }
}
