use std::{collections::VecDeque, time};

use crate::shared::{FrameId, SimCommand, SimInput};

use super::{actor_ids::ActorIndex, cmd_buffer};

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
    pub fn new(capacity: i16, frame_duration: time::Duration) -> Control {
        Control {
            frame_duration,
            time_accumulator: time::Duration::from_micros(0),
            actor_data: Vec::<ActorData>::with_capacity(capacity as usize),
        }
    }

    pub fn add_actor(&mut self, current: FrameId) -> usize {
        self.actor_data.push(ActorData::new(current));
        self.actor_data.len() - 1
    }

    pub fn remove_actor(&mut self, index: ActorIndex) {
        self.actor_data.swap_remove(index);
    }

    pub fn add_commands(
        &mut self,
        actor_index: ActorIndex,
        commands: &[SimCommand],
        frame: FrameId,
    ) {
        let actor = &mut self.actor_data[actor_index];
        actor.cmd_buffer.add_commands(commands, frame);
    }

    pub fn update(&mut self, delta: time::Duration) -> Option<Vec<SimInput>> {
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Control;

    #[test]
    fn update() {
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
}
