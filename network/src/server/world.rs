use crate::shared::{FrameId, SimInput};

use super::actor_ids::ActorIndex;

struct ActorInfo {
    _name: &'static str,
}

pub struct World {
    head: FrameId,
    actor_info: Vec<ActorInfo>,
}

impl World {
    pub fn new(start_frame: FrameId, actor_capacity: i16) -> World {
        World {
            head: start_frame,
            actor_info: Vec::<ActorInfo>::with_capacity(actor_capacity as usize),
        }
    }

    pub fn stop(self) {}

    pub fn step(&mut self, _input: Vec<SimInput>) -> FrameId {
        self.head += 1;
        self.head
    }

    pub fn add_actor(&mut self, name: &'static str) -> usize {
        self.actor_info.push(ActorInfo { _name: name });
        self.actor_info.len() - 1
    }

    pub fn remove_actor(&mut self, index: ActorIndex) {
        self.actor_info.swap_remove(index);
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::SimInput;

    use super::World;

    #[test]
    fn step() {
        let start_frame = 0;
        let mut ctx = World::new(start_frame, 8);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 1);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 2);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 3);

        ctx.stop();
    }
}
