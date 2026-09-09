pub mod test_square;

use crate::frame::ToBytes;

pub trait Generator {
    fn next_frame(&mut self) -> impl ToBytes;
}
