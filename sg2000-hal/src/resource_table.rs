#[repr(u32)]
pub enum FwResourceType {
    RscCarveout = 0,
    RscDevmem = 1,
    RscTrace = 2,
    RscVdev = 3,
    RscLast = 4,
}

#[repr(C)]
pub struct FwRscTrace {
    pub type_: u32,
    pub da: u32,
    pub len: u32,
    pub reserved: u32,
    pub name: [u8; 32],
}

#[repr(C)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offsets: [u32; N],
}

impl<const N: usize> ResourceTableHeader<N> {
    const fn new(offsets: [u32; N]) -> Self {
        ResourceTableHeader {
            ver: 1,
            num: N as u32,
            reserved: [0, 0],
            offsets,
        }
    }
}

pub const TRACE_BUFFER_SIZE: usize = 0x1000;
pub const TRACE_BUFFER_DA: u32 = 0x9ffff000;

#[repr(C)]
pub struct ResourceTable {
    pub header: ResourceTableHeader<1>,
    pub trace: FwRscTrace,
}

const fn name(s: &str) -> [u8; 32] {
    let mut bytes = [0; 32];
    let s_bytes = s.as_bytes();
    let mut i = 0;
    assert!(s_bytes.len() <= 32);
    while i < s_bytes.len() {
        bytes[i] = s_bytes[i];
        i += 1;
    }
    bytes
}

#[unsafe(link_section = ".resource_table")]
#[unsafe(no_mangle)]
#[used]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    header: ResourceTableHeader::new([size_of::<ResourceTableHeader<1>>() as u32]),
    trace: FwRscTrace {
        type_: FwResourceType::RscTrace as u32,
        da: TRACE_BUFFER_DA,
        len: TRACE_BUFFER_SIZE as u32,
        reserved: 0,
        name: name("trace0\0"),
    },
};
