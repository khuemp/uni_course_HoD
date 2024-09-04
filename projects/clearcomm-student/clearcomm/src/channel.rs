use async_std::prelude::*;
use color_eyre::eyre::Result;
use rand::prelude::*;

#[derive(Clone, Copy)]
enum State {
    Good,
    Bad,
}

impl State {
    fn transition(self) -> Self {
        match self {
            State::Good => Self::Bad,
            State::Bad => Self::Good,
        }
    }
}

pub struct Channel {
    state: State,
    h: f64,
    tau: f64,
    channel_bit_errors: u32,
}

impl Channel {
    pub fn new(h: f64, tau: f64) -> Self {
        Self {
            state: State::Good,
            h,
            tau,
            channel_bit_errors: 0,
        }
    }

    pub(super) fn channel_bit_errors(&self) -> u32 {
        self.channel_bit_errors
    }

    pub fn channel_information(&self) -> ChannelInformation {
        ChannelInformation {
            h: self.h,
            tau: self.tau,
        }
    }

    pub async fn process<'a, S: 'a>(&'a mut self, stream: S) -> Result<impl Stream<Item = u8> + 'a>
    where
        S: Stream<Item = u8>,
    {
        let transition = rand::distributions::Uniform::new(0.0, 1.0); //generate transition probability and error rate randomly
        let error = rand::distributions::Uniform::new(0.0, 1.0);

        Ok(stream.map(move |byte| {
            let mut rng = rand::thread_rng();
            let ge_bits = (0..8) // randomly transit between good state and bad state for given transition probability
                .into_iter()
                .map(|_| {
                    if rng.sample(transition) < self.tau {
                        self.state = self.state.transition();
                    }
                    match self.state {
                        State::Good => 0,
                        State::Bad => {
                            // sometimes generate error in bad state
                            if rng.sample(error) < self.h {
                                self.channel_bit_errors += 1;
                                0b01 // plus one byte
                            } else {
                                0
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();
            let ge_byte = super::to_byte(&ge_bits);
            byte ^ ge_byte
        }))
    }
}

#[derive(Debug)]
pub struct ChannelInformation {
    h: f64,
    tau: f64,
}

impl ChannelInformation {
    pub fn get_h(&self) -> f64 {
        self.h
    }

    pub fn get_tau(&self) -> f64 {
        self.tau
    }
}
