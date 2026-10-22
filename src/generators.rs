use crate::frame::GifFrame;

pub mod jel;
pub mod test_square;
pub mod text;
pub mod text_lines;

pub trait Generator: Send + Sync {
    fn next_frame(&mut self) -> Option<GifFrame>;
}
