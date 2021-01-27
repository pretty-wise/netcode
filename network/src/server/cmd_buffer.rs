use crate::shared::{FrameId, SimCommand, SimInput};

use std::collections::VecDeque;

pub struct Buffer {
    data: VecDeque<SimCommand>,
    most_recent: FrameId,
    last_consumed: FrameId,
}

impl Buffer {
    pub fn new(last_frame: FrameId) -> Buffer {
        let mut data = VecDeque::<SimCommand>::new();
        data.push_back(SimCommand::default());
        Buffer {
            data,
            most_recent: last_frame,
            last_consumed: last_frame - 1,
        }
    }

    pub fn add_commands(&mut self, commands: &[SimCommand], most_recent: FrameId) {
        let least_recent = most_recent - (commands.len() as i32 - 1);
        let last_received_cmd = self.data.back().unwrap().clone();

        // fill in missing values
        for _ in self.most_recent + 1..least_recent {
            self.data.push_back(last_received_cmd);
            self.most_recent += 1;
        }

        let oldest_accepted = if self.last_consumed > least_recent {
            self.last_consumed
        } else {
            least_recent
        };
        for frame in oldest_accepted..self.most_recent + 1 {
            let read_index = frame - least_recent;
            let peek_index = frame - (self.last_consumed + 1);
            self.data[peek_index as usize] = commands[read_index as usize];
        }

        // read commands from input
        let mut next_expected = self.most_recent + 1;
        while next_expected <= most_recent {
            let index = next_expected - least_recent;
            self.data.push_back(commands[index as usize]);
            self.most_recent = next_expected;
            next_expected += 1;
        }
    }

    pub fn consume_command(&mut self) -> SimInput {
        if self.data.len() == 1 {
            return SimInput {
                previous: self.data[0],
                current: self.data[0],
            };
        }

        let result = SimInput {
            previous: self.data[0],
            current: self.data[1],
        };
        self.data.pop_front();
        self.last_consumed += 1;
        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::{FrameId, SimCommand, SimInput};

    use super::Buffer;

    #[test]
    fn empty() {
        const START_FRAME: i32 = 0;
        let mut buffer = Buffer::new(START_FRAME);

        assert_eq!(buffer.data.len(), 1); // default command

        let expected_input = SimInput {
            previous: SimCommand::default(),
            current: SimCommand::default(),
        };
        for _ in 1..3 {
            println!("before: {:?}", buffer.data);
            assert_eq!(buffer.consume_command(), expected_input);
            println!(" after: {:?}", buffer.data);
            assert_eq!(
                buffer.most_recent - buffer.last_consumed,
                buffer.data.len() as i32
            );
            assert_eq!(buffer.len(), 1);
        }
    }

    #[test]
    fn inorder() {
        const START_FRAME: i32 = 0;
        let mut buffer = Buffer::new(START_FRAME);

        let total_cmds = buffer.data.len();

        let commands = [
            SimCommand { buttons: 1 },
            SimCommand { buttons: 2 },
            SimCommand { buttons: 3 },
        ];

        buffer.add_commands(&commands, START_FRAME + commands.len() as FrameId);
        println!("initial: {:?}", buffer.data);
        assert_eq!(buffer.data.len(), total_cmds + commands.len());

        let mut prev_cmd = SimCommand::default();
        for (i, cmd) in commands.iter().enumerate() {
            let expected_input = SimInput {
                previous: prev_cmd,
                current: cmd.clone(),
            };

            println!("before: {:?}", buffer.data);
            let input = buffer.consume_command();
            println!(" after: {:?}", buffer.data);
            assert_eq!(input, expected_input, "at index {}", i);

            prev_cmd = expected_input.current;
        }

        println!("before: {:?}", buffer.data);
        let input = buffer.consume_command();
        println!(" after: {:?}", buffer.data);
        assert_eq!(
            input,
            SimInput {
                previous: commands[commands.len() - 1],
                current: commands[commands.len() - 1]
            }
        );
    }

    #[test]
    fn with_gap() {
        const START_FRAME: i32 = 0;
        let mut buffer = Buffer::new(START_FRAME);

        let total_cmds = buffer.data.len();

        let initial_cmds = [
            SimCommand { buttons: 1 },
            SimCommand { buttons: 2 },
            SimCommand { buttons: 3 },
        ];

        buffer.add_commands(&initial_cmds, START_FRAME + initial_cmds.len() as FrameId);

        let followup_cmds = [
            SimCommand { buttons: 6 },
            SimCommand { buttons: 7 },
            SimCommand { buttons: 8 },
        ];

        let gap_size = 2;
        buffer.add_commands(
            &followup_cmds,
            START_FRAME + initial_cmds.len() as FrameId + followup_cmds.len() as FrameId + gap_size,
        );

        let mut prev_cmd = SimCommand::default();
        for (i, cmd) in initial_cmds.iter().enumerate() {
            let expected_input = SimInput {
                previous: prev_cmd,
                current: cmd.clone(),
            };

            println!("before: {:?}", buffer.data);
            let input = buffer.consume_command();
            println!(" after: {:?}", buffer.data);
            assert_eq!(input, expected_input, "at index {}", i);

            prev_cmd = expected_input.current;
        }

        let last_cmd = initial_cmds[initial_cmds.len() - 1];
        for i in 0..gap_size {
            let expected_input = SimInput {
                previous: last_cmd,
                current: last_cmd,
            };

            let input = buffer.consume_command();

            assert_eq!(input, expected_input);
        }

        let mut prev_cmd = last_cmd;
        for (i, cmd) in followup_cmds.iter().enumerate() {
            let expected_input = SimInput {
                previous: prev_cmd,
                current: cmd.clone(),
            };

            println!("before: {:?}", buffer.data);
            let input = buffer.consume_command();
            println!(" after: {:?}", buffer.data);
            assert_eq!(input, expected_input, "at index {}", i);

            prev_cmd = expected_input.current;
        }

        println!("before: {:?}", buffer.data);
        let input = buffer.consume_command();
        println!(" after: {:?}", buffer.data);
        assert_eq!(
            input,
            SimInput {
                previous: followup_cmds[initial_cmds.len() - 1],
                current: followup_cmds[initial_cmds.len() - 1]
            }
        );
    }

    #[test]
    fn gap_fill() {
        const START_FRAME: i32 = 0;
        let mut buffer = Buffer::new(START_FRAME);

        let total_cmds = buffer.data.len();

        let initial_cmds = [
            SimCommand { buttons: 1 },
            SimCommand { buttons: 2 },
            SimCommand { buttons: 3 },
        ];

        buffer.add_commands(&initial_cmds, START_FRAME + initial_cmds.len() as FrameId);

        let followup_cmds = [
            SimCommand { buttons: 6 },
            SimCommand { buttons: 7 },
            SimCommand { buttons: 8 },
        ];

        let gap_size = 2;
        buffer.add_commands(
            &followup_cmds,
            START_FRAME + initial_cmds.len() as FrameId + followup_cmds.len() as FrameId + gap_size,
        );

        let overlapping_cmds = [
            SimCommand { buttons: 3 },
            SimCommand { buttons: 4 },
            SimCommand { buttons: 5 },
            SimCommand { buttons: 6 },
            SimCommand { buttons: 7 },
            SimCommand { buttons: 8 },
            SimCommand { buttons: 9 },
        ];

        let expected_cmds = [
            SimCommand { buttons: 1 },
            SimCommand { buttons: 2 },
            SimCommand { buttons: 3 },
            SimCommand { buttons: 4 },
            SimCommand { buttons: 5 },
            SimCommand { buttons: 6 },
            SimCommand { buttons: 7 },
            SimCommand { buttons: 8 },
            SimCommand { buttons: 9 },
        ];

        println!("before: {:?}", buffer.data);
        buffer.add_commands(
            &overlapping_cmds,
            START_FRAME + expected_cmds.len() as FrameId,
        );
        println!(" after: {:?}", buffer.data);

        let mut prev_cmd = SimCommand::default();
        for i in 0..expected_cmds.len() {
            let expected = SimInput {
                previous: prev_cmd,
                current: expected_cmds[i],
            };
            println!("before: {:?}", buffer.data);
            let input = buffer.consume_command();
            println!(" after: {:?}", buffer.data);
            assert_eq!(input, expected);

            prev_cmd = input.current;
        }

        println!("before: {:?}", buffer.data);
        let input = buffer.consume_command();
        println!(" after: {:?}", buffer.data);
        assert_eq!(
            input,
            SimInput {
                previous: expected_cmds[expected_cmds.len() - 1],
                current: expected_cmds[expected_cmds.len() - 1]
            }
        );
    }
}
