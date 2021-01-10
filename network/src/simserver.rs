use crate::simshared::FrameId;

pub struct Context {
    head: FrameId,
}

impl Context {
    pub fn start() -> Context {
        Context { head: 0 }
    }

    pub fn stop(self) {}

    pub fn step(&mut self) {
        self.head += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    #[test]
    fn creation() {
        let ctx = Context::start();
        ctx.stop();
    }
}
