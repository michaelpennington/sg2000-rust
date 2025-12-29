use crate::clock::{ClockSource, Rate};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct Config {
    clock: SourceClock,
    baud_rate: u32,
    data_len: DataLen,
    stop_bits: StopBits,
    parity: ParityMode,
    add_cr: bool,
}

impl Config {
    pub fn clock(&self) -> SourceClock {
        self.clock
    }

    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn data_len(&self) -> DataLen {
        self.data_len
    }

    pub fn stop_bits(&self) -> StopBits {
        self.stop_bits
    }

    pub fn parity(&self) -> ParityMode {
        self.parity
    }

    pub fn add_cr(&self) -> bool {
        self.add_cr
    }

    pub fn with_clock(mut self, clock: SourceClock) -> Self {
        self.clock = clock;
        self
    }

    pub fn with_baud_rate(mut self, baud_rate: u32) -> Self {
        self.baud_rate = baud_rate;
        self
    }

    pub fn with_data_len(mut self, data_len: DataLen) -> Self {
        self.data_len = data_len;
        self
    }

    pub fn with_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    pub fn with_parity(mut self, parity: ParityMode) -> Self {
        self.parity = parity;
        self
    }

    pub fn with_add_cr(mut self, add_cr: bool) -> Self {
        self.add_cr = add_cr;
        self
    }

    pub(crate) fn validate(&self) -> bool {
        self.stop_bits == StopBits::One
            || self.stop_bits == StopBits::OnePFive && self.data_len == DataLen::Five
            || self.stop_bits == StopBits::Two && self.data_len != DataLen::Five
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock: Default::default(),
            baud_rate: 115200,
            data_len: Default::default(),
            stop_bits: Default::default(),
            parity: Default::default(),
            add_cr: false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SourceClock {
    #[default]
    Xtal,
    // DispPLL,
}

impl ClockSource for SourceClock {
    fn hz(&self) -> Rate {
        match self {
            SourceClock::Xtal => Rate::from_khz(25_000),
            // SourceClock::DispPLL => Rate::from_khz(187_500),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DataLen {
    Five,
    Six,
    Seven,
    #[default]
    Eight,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One,
    OnePFive,
    #[default]
    Two,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ParityMode {
    #[default]
    None,
    Odd,
    Even,
    Mark,
    Space,
}
