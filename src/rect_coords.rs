use num::Num;

#[derive(Clone, Copy)]
pub struct RectCoords<T: Num> {
    pub left_x: T,
    pub top_y: T,
    pub right_x: T,
    pub bottom_y: T,
}
