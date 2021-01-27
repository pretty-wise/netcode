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

    pub fn step(&mut self, input: Vec<SimInput>) -> FrameId {
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

    pub fn head(&self) -> FrameId {
        self.head
    }
}
