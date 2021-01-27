use std::time;

use crate::shared::types::FrameId;

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
    ) -> Simulation {
        Simulation {
            ids: ActorIds::new(capacity),
            main_world: World::new(start_frame, capacity),
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

    pub fn add_actor(&mut self, current: FrameId, name: &'static str) -> Option<ActorId> {
        let (id, _) = self.ids.add()?;

        self.control.add_actor(current);
        self.main_world.add_actor(name);
        Some(id)
    }

    pub fn remove_actor(&mut self, id: ActorId) {
        match self.ids.remove(id) {
            Some(index) => {
                self.control.remove_actor(index);
                self.main_world.remove_actor(index);
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        server::world::World,
        shared::{FrameId, SimInput},
    };

    use super::Control;
    use std::time::Duration;

    #[test]
    fn control_update() {
        let frame_duration = Duration::from_millis(16);
        let mut ctx = Control::new(2, frame_duration);
        assert!(ctx.update(frame_duration).is_some());

        assert!(ctx
            .update(frame_duration - Duration::from_millis(1))
            .is_none());

        assert!(ctx.update(Duration::from_millis(1)).is_some());

        assert!(ctx.update(2 * frame_duration).is_some());
        assert!(ctx.update(Duration::from_millis(0)).is_some());
        assert!(ctx.update(Duration::from_millis(0)).is_none());
    }

    #[test]
    fn context_step() {
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

    #[test]
    fn input_size() {
        const CAPACITY: i16 = 2;
        const DELTA: Duration = Duration::from_millis(16);
        const START_FRAME: FrameId = 0;
        let mut ctrl = Control::new(CAPACITY, DELTA);
        let mut world = World::new(START_FRAME, CAPACITY);

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
