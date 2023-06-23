use raylib::prelude::RaylibDrawHandle;

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub x: i32,
    pub y: i32,
    pub w: Option<i32>,
    pub h: Option<i32>,
}

impl Dimensions {
    pub fn new(x: i32, y: i32, w: Option<i32>, h: Option<i32>) -> Dimensions {
        Dimensions { x, y, w, h }
    }

    pub fn draw_border(&self, handle: &mut RaylibDrawHandle) {}

    pub fn draw_inner(&self, handle: &mut RaylibDrawHandle) {}
}
