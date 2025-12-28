pub trait ClockSource {
    fn hz(&self) -> Rate;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rate(u64);

impl Rate {
    pub fn from_hz(hz: u64) -> Self {
        Self(hz)
    }

    pub fn from_khz(khz: u64) -> Self {
        Self(khz * 1_000)
    }

    pub fn from_mhz(mhz: u64) -> Self {
        Self(mhz * 1_000_000)
    }

    pub fn from_ghz(ghz: u64) -> Self {
        Self(ghz * 1_000_000_000)
    }

    pub fn as_hz(&self) -> u64 {
        self.0
    }

    pub fn as_khz(&self) -> u64 {
        self.0 / 1_000
    }

    pub fn as_mhz(&self) -> u64 {
        self.0 / 1_000_000
    }

    pub fn as_ghz(&self) -> u64 {
        self.0 / 1_000_000_000
    }
}
