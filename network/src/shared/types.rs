pub type FrameId = i32;
pub const INVALID_FRAMEID: FrameId = 0;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SimCommand {
    pub buttons: i32,
}

impl SimCommand {
    pub fn default() -> SimCommand {
        SimCommand { buttons: 0 }
    }
}
#[derive(PartialEq, Debug)]
pub struct SimInput {
    pub previous: SimCommand,
    pub current: SimCommand,
}
