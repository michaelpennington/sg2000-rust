use core::sync::atomic::{Ordering, fence};

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
    pub last_avail_idx: u16,
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
            last_avail_idx: 0,
        }
    }

    pub fn get_avail_buf(&mut self) -> Option<u16> {
        let avail_idx = unsafe { (*self.avail).idx };
        fence(Ordering::SeqCst);
        if self.last_avail_idx != avail_idx {
            let ring_idx = (self.last_avail_idx % self.num) as usize;
            let desc_idx = unsafe { (*self.avail).ring[ring_idx] };
            self.last_avail_idx = self.last_avail_idx.wrapping_add(1);
            Some(desc_idx)
        } else {
            None
        }
    }

    pub fn add_used_buf(&mut self, desc_idx: u16, len: u32) {
        let used_idx = unsafe { (*self.used).idx };
        let ring_idx = (used_idx % self.num) as usize;

        unsafe {
            (*self.used).ring[ring_idx] = VRingUsedElem {
                id: desc_idx as u32,
                len,
            };
        }
        fence(Ordering::SeqCst);
        unsafe {
            (*self.used).idx = used_idx.wrapping_add(1);
        }
    }

    /// # Safety
    ///
    /// desc_idx must be valid
    pub unsafe fn get_buf_slice(&mut self, desc_idx: u16) -> &mut [u8] {
        unsafe {
            let desc = *self.desc.add(desc_idx as usize);
            core::slice::from_raw_parts_mut(desc.addr as *mut u8, desc.len as usize)
        }
    }
}
