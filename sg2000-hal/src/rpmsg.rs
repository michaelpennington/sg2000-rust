#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RpmsgHeader {
    pub src: u32,
    pub dst: u32,
    pub reserved: u32,
    pub len: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RpmsgNsMsg {
    pub name: [u8; 32],
    pub addr: u32,
    pub flags: u32,
}

impl RpmsgNsMsg {
    pub fn new(name: &str, addr: u32) -> Self {
        let mut name_bytes = [0u8; 32];
        let bytes = name.as_bytes();
        let len = bytes.len().min(32);
        name_bytes[..len].copy_from_slice(&bytes[..len]);

        Self {
            name: name_bytes,
            addr,
            flags: 3,
        }
    }
}

pub const RPMSG_ADDR_NS: u32 = 53;
pub const RPMSG_RESERVED: u32 = 0;
