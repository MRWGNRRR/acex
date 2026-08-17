extern crate ace_macros;

use acex_core::DiagError;
use acex_macros::FrameCodec;

#[derive(Debug)]
pub struct TestError;
impl From<TestError> for DiagError {
    fn from(_: TestError) -> Self {
        // was: _: DiagError
        DiagError::InvalidFrame(heapless::String::new())
    }
}

impl From<DiagError> for TestError {
    fn from(_: DiagError) -> Self {
        TestError
    }
}

#[derive(Debug, PartialEq, FrameCodec)]
#[frame(error = "TestError")]
pub struct WithLength<'a> {
    pub data_length: u8,
    #[frame(length = "data_length as usize")]
    pub data: &'a [u8],
    pub trailing: u8,
}

fn main() {}
