use std::{num::NonZeroI16, time};

use crate::shared::{types::FrameId, SimCommand};

use super::world::World;
use super::{
    actor_ids::{ActorId, ActorIds},
    control::Control,
};

pub struct Simulation {
    ids: ActorIds,
    main_world: World,
    control: Control,
}

impl Simulation {
    pub fn start(
        start_frame: FrameId,
        frame_duration: time::Duration,
        capacity: i16,
        object_cap: i16,
    ) -> Simulation {
        Simulation {
            ids: ActorIds::new(capacity),
            main_world: World::new(start_frame, capacity, object_cap),
            control: Control::new(capacity, frame_duration),
        }
    }

    pub fn update(&mut self, delta: time::Duration) {
        let mut step_time = delta;

        while let Some(inputs) = self.control.update(step_time) {
            let _new_frame_id = self.main_world.step(inputs);

            step_time = time::Duration::from_millis(0);
        }
    }

    pub fn stop(self) {}

    pub fn read(&mut self, buffer: &[u8]) {
        let current: FrameId = 0;

        // parse buffer and push to ctrl and main_world.
        if buffer[0] == 0 {
            self.add_actor(current, "test");
        } else if buffer[0] == 1 {
            let actor: ActorId = NonZeroI16::new(1).unwrap();
            self.remove_actor(actor);
        } else if buffer[0] == 2 {
            let actor: ActorId = NonZeroI16::new(1).unwrap();
            if let Some(actor_index) = self.ids.find_index(actor) {
                let commands = [SimCommand { buttons: 0 }];
                self.control.add_commands(actor_index, &commands, current);
            }
        }
    }

    pub fn add_actor(&mut self, current: FrameId, name: &'static str) -> Option<ActorId> {
        let (id, _) = self.ids.add()?;

        self.control.add_actor(current);
        self.main_world.add_actor(name);
        Some(id)
    }

    pub fn remove_actor(&mut self, id: ActorId) {
        if let Some(index) = self.ids.remove(id) {
            self.control.remove_actor(index);
            self.main_world.remove_actor(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{server::world::World, shared::FrameId};

    use super::Control;
    use std::time::Duration;

    #[test]
    fn ctrl_world_integration() {
        const CAPACITY: i16 = 2;
        const DELTA: Duration = Duration::from_millis(16);
        const START_FRAME: FrameId = 0;
        let mut ctrl = Control::new(CAPACITY, DELTA);
        let mut world = World::new(START_FRAME, CAPACITY, 0);

        assert_eq!(ctrl.add_actor(START_FRAME), world.add_actor("first"));

        let inputs = ctrl.update(DELTA);
        assert!(inputs.is_some());
        let inputs = inputs.unwrap();
        assert_eq!(inputs.len(), 1);

        let current_frame = world.step(inputs);

        assert_eq!(ctrl.add_actor(current_frame), world.add_actor("second"));

        let inputs = ctrl.update(DELTA);
        assert!(inputs.is_some());
        let inputs = inputs.unwrap();
        assert_eq!(inputs.len(), 2);

        world.step(inputs);
    }
}
