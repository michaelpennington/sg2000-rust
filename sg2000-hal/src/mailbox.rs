pub struct Mailbox {
    // hw_lock: RawSpinlock,
    channel: u8,
}

const NUM_CHANNELS: u8 = 8;

// struct RawSpinlock;

impl Mailbox {
    pub fn new(channel: u8) -> Self {
        assert!(channel < NUM_CHANNELS);
        Self { channel }
    }
}
