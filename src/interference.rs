use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct InterferenceDetector {
    channel_states: Vec<ChannelState>,
}

#[derive(Clone)]
struct ChannelState {
    interference_count: u32,
    last_interference: Instant,
    success_count: u32,
}

impl InterferenceDetector {
    pub fn new(num_channels: usize) -> Self {
        InterferenceDetector {
            channel_states: vec![ChannelState::new(); num_channels],
        }
    }

    pub fn is_channel_interfered(&self, index: usize) -> bool {
        let state = &self.channel_states[index];
        state.interference_count > state.success_count
            && state.last_interference.elapsed() < Duration::from_secs(60)
    }

    pub fn report_interference(&mut self, index: usize) {
        let state = &mut self.channel_states[index];
        state.interference_count += 1;
        state.last_interference = Instant::now();
    }

    pub fn report_success(&mut self, index: usize) {
        let state = &mut self.channel_states[index];
        state.success_count += 1;
    }

    pub fn least_interfered_channel(&self) -> usize {
        self.channel_states
            .iter()
            .enumerate()
            .min_by_key(|(_, state)| state.interference_count)
            .map(|(index, _)| index)
            .unwrap_or(0)
    }
}

impl ChannelState {
    fn new() -> Self {
        ChannelState {
            interference_count: 0,
            last_interference: Instant::now(),
            success_count: 0,
        }
    }
}
