#[repr(u32)]
pub enum FwResourceType {
    RscCarveout = 0,
    RscDevmem = 1,
    RscTrace = 2,
    RscVdev = 3,
    RscLast = 4,
}

pub const VIRTIO_ID_RPMSG: u32 = 7;

pub const VIRTIO_RPMSG_F_NS: u32 = 0; // Name Service feature bit
pub const VRING_NUM: u32 = 512; // Number of descriptors (must match host expectation or be negotiated)
pub const VRING_ALIGN: u32 = 4096;

#[repr(C)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offsets: [u32; N],
}

/// Defines the VirtIO device header in the resource table
#[repr(C)]
pub struct FwRscVdev {
    pub type_: u32,
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub reserved: [u8; 2],
}

/// Defines a specific vring within the VDEV
#[repr(C)]
pub struct FwRscVdevVring {
    pub da: u32, // Device Address (0 means Host allocates)
    pub align: u32,
    pub num: u32,
    pub notifyid: u32, // Mailbox payload sent by Host to kick this ring
    pub reserved: u32,
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

#[repr(C)]
pub struct ResourceTable {
    pub header: ResourceTableHeader<1>,
    pub rpmsg_vdev: FwRscVdev,
    pub rpmsg_vring0: FwRscVdevVring, // RX Vring
    pub rpmsg_vring1: FwRscVdevVring, // TX Vring
}

#[unsafe(link_section = ".resource_table")]
#[unsafe(no_mangle)]
#[used]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    header: ResourceTableHeader::new([size_of::<ResourceTableHeader<1>>() as u32]),
    rpmsg_vdev: FwRscVdev {
        type_: FwResourceType::RscVdev as u32,
        id: VIRTIO_ID_RPMSG,
        notifyid: 0,
        dfeatures: 1 << VIRTIO_RPMSG_F_NS,
        gfeatures: 0,
        config_len: 0,
        status: 0,
        num_of_vrings: 2,
        reserved: [0; 2],
    },
    rpmsg_vring0: FwRscVdevVring {
        da: 0,
        align: VRING_ALIGN,
        num: VRING_NUM,
        notifyid: 0,
        reserved: 0,
    },
    rpmsg_vring1: FwRscVdevVring {
        da: 0,
        align: VRING_ALIGN,
        num: VRING_NUM,
        notifyid: 1,
        reserved: 0,
    },
};
