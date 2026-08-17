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
pub struct WithSkip {
    pub real: u8,
    #[frame(skip)]
    pub internal: u8,
    pub also_real: u8,
}

fn main() {}
