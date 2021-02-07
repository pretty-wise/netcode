use crate::shared::{FrameId, ObjectId, SimInput};

use super::actor_ids::ActorIndex;

struct ActorInfo {
    _name: &'static str,
}

pub struct World {
    head: FrameId,
    actor_info: Vec<ActorInfo>,
    objects: Vec<ObjectId>,
    id_generator: ObjectId,
}

impl World {
    pub fn new(start_frame: FrameId, actor_capacity: i16, object_capacity: i16) -> World {
        World {
            head: start_frame,
            actor_info: Vec::<ActorInfo>::with_capacity(actor_capacity as usize),
            objects: Vec::<ObjectId>::with_capacity(object_capacity as usize),
            id_generator: 0,
        }
    }

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

    pub fn add_object(&mut self) -> Option<ObjectId> {
        if self.objects.capacity() == self.objects.len() {
            return None;
        }

        self.id_generator += 1;
        let new_id = self.id_generator;
        self.objects.push(new_id);
        Some(new_id)
    }

    pub fn remove_object(&mut self, id: ObjectId) -> bool {
        if let Some(index) = self.objects.iter().position(|&value| value == id) {
            self.objects.swap_remove(index);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::SimInput;

    use super::World;

    #[test]
    fn step() {
        let start_frame = 0;
        let mut ctx = World::new(start_frame, 8, 0);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 1);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 2);
        let input = Vec::<SimInput>::new();
        assert_eq!(ctx.step(input), start_frame + 3);
    }

    #[test]
    fn object_creation() {
        let mut ctx = World::new(0, 8, 1);
        let obj = ctx.add_object();
        assert!(obj.is_some());
        assert_eq!(ctx.remove_object(obj.unwrap()), true);

        let obj = ctx.add_object();
        assert!(obj.is_some());

        let obj = ctx.add_object();
        assert!(obj.is_none());
    }
}
