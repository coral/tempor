use anyhow::{anyhow, Result};
use std::time::{Duration, Instant};

pub struct Tapper {
    lookback: usize,
    min_taps: usize,
    cooldown: Duration,
    taps: Vec<Instant>,
}

impl Tapper {
    pub fn new(min_taps: usize, cooldown: Duration) -> Result<Tapper> {
        if min_taps < 2 {
            return Err(anyhow!(
                "min_taps cannot be less than 2, currently: {}",
                min_taps
            ));
        }

        Ok(Tapper {
            lookback: 16,
            min_taps: min_taps,
            cooldown: cooldown,
            taps: Vec::new(),
        })
    }

    pub fn tap(&mut self) -> Option<f64> {
        let n = Instant::now();

        match self.taps.last() {
            Some(t) => {
                if n.duration_since(*t) > self.cooldown {
                    self.taps.clear()
                }
            }
            None => {}
        }

        self.taps.push(n);

        //If we have less taps than the defined "min_taps", exit early with None.
        if self.taps.len() < self.min_taps {
            return None;
        }

        //Clear the backlog
        if self.taps.len() >= self.lookback {
            self.taps.remove(0);
        }

        let r = self
            .taps
            .iter()
            .zip(self.taps.iter().skip(1))
            .map(|(prev, current)| current.duration_since(*prev).as_micros())
            .collect::<Vec<u128>>();

        let bpm = Duration::from_secs(60).as_micros() as f64 / mean(&r);

        return Some(bpm);
    }

    pub fn clear(&mut self) {
        self.taps.clear();
    }
}

fn mean(list: &[u128]) -> f64 {
    let sum: u128 = Iterator::sum(list.iter());
    sum as f64 / (list.len() as f64)
}
