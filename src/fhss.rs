use crate::interference::InterferenceDetector;
use rand::Rng;
use std::collections::VecDeque;

pub struct FrequencyHopper {
    frequencies: Vec<f64>,
    pub current_index: usize,
    history: VecDeque<usize>,
    interference_detector: InterferenceDetector,
}

impl FrequencyHopper {
    pub fn new(min_freq: f64, max_freq: f64, num_channels: usize) -> Self {
        let mut rng = rand::thread_rng();
        let frequencies: Vec<f64> = (0..num_channels)
            .map(|_| rng.gen_range(min_freq..max_freq))
            .collect();

        FrequencyHopper {
            frequencies,
            current_index: 0,
            history: VecDeque::with_capacity(num_channels),
            interference_detector: InterferenceDetector::new(num_channels),
        }
    }

    pub fn next_frequency(&mut self) -> f64 {
        let mut attempts = 0;
        loop {
            let index = self.select_channel();
            let freq = self.frequencies[index];

            if !self.interference_detector.is_channel_interfered(index) {
                self.update_history(index);
                return freq;
            }

            attempts += 1;
            if attempts >= self.frequencies.len() {
                // If all channels are interfered, use the least interfered one
                let least_interfered = self.interference_detector.least_interfered_channel();
                self.update_history(least_interfered);
                return self.frequencies[least_interfered];
            }
        }
    }

    fn select_channel(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.frequencies.len())
    }

    fn update_history(&mut self, index: usize) {
        if self.history.len() == self.history.capacity() {
            self.history.pop_front();
        }
        self.history.push_back(index);
        self.current_index = index;
    }

    pub fn report_interference(&mut self, index: usize) {
        self.interference_detector.report_interference(index);
    }

    pub fn report_success(&mut self, index: usize) {
        self.interference_detector.report_success(index);
    }
}
