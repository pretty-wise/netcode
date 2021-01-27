use std::{collections::VecDeque, time};

use crate::shared::types::{FrameId, SimCommand, SimInput};

use super::actor_ids::{self, ActorId, ActorIndex};
use super::cmd_buffer;

struct ActorData {
    // todo: remote entity id
    cmd_buffer: cmd_buffer::Buffer,
}

impl ActorData {
    fn new(last_frame: FrameId) -> ActorData {
        let mut input_buffer = VecDeque::<SimCommand>::new();
        input_buffer.push_back(SimCommand::default());
        ActorData {
            cmd_buffer: cmd_buffer::Buffer::new(last_frame),
        }
    }
}

pub struct Control {
    frame_duration: time::Duration,
    time_accumulator: time::Duration,
    actor_data: Vec<ActorData>,
}

impl Control {
    fn new(capacity: i16, frame_duration: time::Duration) -> Control {
        Control {
            frame_duration,
            time_accumulator: time::Duration::from_micros(0),
            actor_data: Vec::<ActorData>::with_capacity(capacity as usize),
        }
    }

    fn add_actor(&mut self, current: FrameId) -> usize {
        self.actor_data.push(ActorData::new(current));
        self.actor_data.len() - 1
    }
    fn remove_actor(&mut self, index: ActorIndex) {
        self.actor_data.swap_remove(index);
    }
    fn add_commands(&mut self, actor_index: ActorIndex, commands: &[SimCommand], frame: FrameId) {
        let actor = &mut self.actor_data[actor_index];
        actor.cmd_buffer.add_commands(commands, frame);
    }
    fn _remove_stale_input(&mut self, frame: FrameId) {
        //
    }
    fn update(&mut self, delta: time::Duration, head: FrameId) -> Option<Vec<SimInput>> {
        self.time_accumulator += delta;
        match self.time_accumulator >= self.frame_duration {
            true => {
                self.time_accumulator -= self.frame_duration;

                let inputs = self
                    .actor_data
                    .iter_mut()
                    .map(|actor| actor.cmd_buffer.consume_command())
                    .collect();
                Some(inputs)
            }
            false => None,
        }
    }
}

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
}

pub struct Simulation {
    ids: actor_ids::ActorIds,
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
            ids: actor_ids::ActorIds::new(capacity),
            main_world: World::new(start_frame, capacity),
            control: Control::new(capacity, frame_duration),
        }
    }

    pub fn update(&mut self, delta: time::Duration) {
        let mut step_time = delta;

        while let Some(inputs) = self.control.update(step_time, self.main_world.head + 1) {
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
    use crate::shared::{FrameId, INVALID_FRAMEID};

    use super::Control;
    use super::SimInput;
    use super::World;
    use std::time::Duration;

    #[test]
    fn control_update() {
        let frame_duration = Duration::from_millis(16);
        let mut ctx = Control::new(2, frame_duration);
        assert!(ctx.update(frame_duration, INVALID_FRAMEID).is_some());

        assert!(ctx
            .update(frame_duration - Duration::from_millis(1), INVALID_FRAMEID)
            .is_none());

        assert!(ctx
            .update(Duration::from_millis(1), INVALID_FRAMEID)
            .is_some());

        assert!(ctx.update(2 * frame_duration, INVALID_FRAMEID).is_some());
        assert!(ctx
            .update(Duration::from_millis(0), INVALID_FRAMEID)
            .is_some());
        assert!(ctx
            .update(Duration::from_millis(0), INVALID_FRAMEID)
            .is_none());
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

        let inputs = ctrl.update(DELTA, world.head + 1);
        assert!(inputs.is_some());
        let inputs = inputs.unwrap();
        assert_eq!(inputs.len(), 1);

        let current_frame = world.step(inputs);

        assert_eq!(ctrl.add_actor(current_frame), world.add_actor("second"));

        let inputs = ctrl.update(DELTA, world.head + 1);
        assert!(inputs.is_some());
        let inputs = inputs.unwrap();
        assert_eq!(inputs.len(), 2);

        world.step(inputs);
    }
}
