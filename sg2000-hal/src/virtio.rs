use crate::resource_table::VRING_NUM;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VRingDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct VRingAvail {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; VRING_NUM as usize],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VRingUsedElem {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct VRingUsed {
    pub flags: u16,
    pub idx: u16,
    pub ring: [VRingUsedElem; VRING_NUM as usize],
}

#[derive(Debug)]
pub struct VirtQueue {
    pub desc: *mut VRingDesc,
    pub avail: *mut VRingAvail,
    pub used: *mut VRingUsed,
    pub num: u16,
    pub last_used_idx: u16,
}

impl VirtQueue {
    /// # Safety
    ///
    /// `base_addr` must be the valid `da` (Device Address) written by the Host
    /// into the Resource Table.
    pub unsafe fn from_resource_table_addr(base_addr: u32, num: u16, align: u32) -> Self {
        let desc = base_addr as *mut VRingDesc;

        let num_usize = num as usize;
        let desc_size = size_of::<VRingDesc>() * num_usize;
        let avail_size = size_of::<VRingAvail>();

        let avail = (base_addr + desc_size as u32) as *mut VRingAvail;

        let used_offset =
            (base_addr + desc_size as u32 + avail_size as u32 + align - 1) & !(align - 1);

        let used = used_offset as *mut VRingUsed;

        VirtQueue {
            desc,
            avail,
            used,
            num,
            last_used_idx: 0,
        }
    }
}
