#[repr(u32)]
pub enum FwResourceType {
    RscCarveout = 0,
    RscDevmem = 1,
    RscTrace = 2,
    RscVdev = 3,
    RscLast = 4,
}

const VIRTIO_ID_CONSOLE: u32 = 3;

#[repr(C)]
pub struct ResourceVring {
    pub da: u32,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub pa: u32,
}

#[repr(C)]
pub struct ConsoleConfig {
    pub cols: u16,
    pub rows: u16,
    pub max_nr_ports: u32,
    pub emerg_wr: u32,
}

#[repr(C)]
pub struct ResourceVdev {
    pub type_: u32,
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub reserved: [u8; 2],
    pub vrings: [ResourceVring; 2], // Queue 0: RX (Host->Guest), Queue 1: TX (Guest->Host)
    // The config struct MUST follow the vrings immediately
    pub config: ConsoleConfig,
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

#[repr(C, align(16))]
pub struct ResourceTable {
    pub header: ResourceTableHeader<1>,
    pub console_vdev: ResourceVdev,
}

#[unsafe(link_section = ".resource_table")]
#[unsafe(no_mangle)]
#[used]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    header: ResourceTableHeader::new([size_of::<ResourceTableHeader<1>>() as u32]),
    console_vdev: ResourceVdev {
        type_: FwResourceType::RscVdev as u32,
        id: VIRTIO_ID_CONSOLE,
        notifyid: 0,
        dfeatures: 0,
        gfeatures: 0,
        config_len: size_of::<ConsoleConfig>() as u32,
        status: 0,
        num_of_vrings: 2,
        reserved: [0; 2],
        vrings: [
            ResourceVring {
                da: 0x8f528000,
                align: 4096,
                num: 16,
                notifyid: 0,
                pa: 0x8f528000,
            },
            ResourceVring {
                da: 0x8f52c000,
                align: 4096,
                num: 16,
                notifyid: 1,
                pa: 0x8f52c000,
            },
        ],
        config: ConsoleConfig {
            cols: 0,
            rows: 0,
            max_nr_ports: 1,
            emerg_wr: 0,
        },
    },
};
