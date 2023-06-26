use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub colour: Color,
    pub border_colour: Color,
    pub border_offset: i32,
}

impl Entity {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        colour: Color,
        border_colour: Color,
        border_offset: i32,
    ) -> Entity {
        Entity {
            x,
            y,
            width,
            height,
            colour,
            border_colour,
            border_offset,
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        // Draw Border
        handle.draw_rectangle(self.x, self.y, self.width, self.height, self.border_colour);

        // Draw Inner
        handle.draw_rectangle(
            self.x + self.border_offset,
            self.y + self.border_offset,
            self.width - (self.border_offset * 2),
            self.height - (self.border_offset * 2),
            self.colour,
        );
    }

    pub fn contains(&self, pos: Vector2) -> bool {
        self.x as f32 <= pos.x
            && (self.x + self.width) as f32 >= pos.x
            && self.y as f32 <= pos.y
            && (self.y + self.height) as f32 >= pos.y
    }
}
