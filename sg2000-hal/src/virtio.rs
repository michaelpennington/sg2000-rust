// sg2000-hal/src/virtio.rs
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{Ordering, fence};

// Standard VirtIO Descriptor Constants

#[repr(C)]
pub struct VirtqDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct VirtqAvail {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; 16], // Size matches 'num: 16' in resource_table
    pub used_event: u16, // Only if VIRTIO_F_EVENT_IDX is negotiated
}

#[repr(C)]
pub struct VirtqUsedElem {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
pub struct VirtqUsed {
    pub flags: u16,
    pub idx: u16,
    pub ring: [VirtqUsedElem; 16],
    pub avail_event: u16,
}

pub struct VirtQueue {
    desc: *mut VirtqDesc,
    avail: *mut VirtqAvail,
    used: *mut VirtqUsed,
    last_avail_idx: u16,
    num: u16,
}

impl VirtQueue {
    /// Initialize the queue based on the base address and alignment from Resource Table
    /// # Safety
    ///
    /// asd
    pub unsafe fn new(base_addr: u32, num: u16, align: u32) -> Self {
        // Calculation logic for Split Virtqueues:
        // Descriptor Table: base
        // Available Ring: base + num * sizeof(Desc)
        // Used Ring: aligned(base + num*sizeof(Desc) + sizeof(Avail))

        let desc_size = 16; // sizeof(VirtqDesc)
        let avail_size = 6 + 2 * num as u32; // sizeof(VirtqAvail) without padding

        let avail_offset = num as u32 * desc_size;
        let used_offset_unaligned = avail_offset + avail_size + 2; // +2 for used_event padding
        let used_offset = (used_offset_unaligned + (align - 1)) & !(align - 1);

        VirtQueue {
            desc: base_addr as *mut VirtqDesc,
            avail: (base_addr + avail_offset) as *mut VirtqAvail,
            used: (base_addr + used_offset) as *mut VirtqUsed,
            last_avail_idx: 0,
            num,
        }
    }

    /// Check if the Driver (Linux) has made new buffers available
    pub fn is_available(&self) -> bool {
        let avail_idx = unsafe { read_volatile(&(*self.avail).idx) };
        avail_idx != self.last_avail_idx
    }

    /// Consumes a buffer from the Available ring.
    /// Returns the (descriptor_index, address, length) if available.
    pub fn get_available_buf(&mut self) -> Option<(u16, *mut u8, u32)> {
        if !self.is_available() {
            return None;
        }

        // 1. Read the descriptor index from the Available Ring
        let ring_idx = (self.last_avail_idx % self.num) as usize;
        let desc_idx = unsafe { read_volatile(&(*self.avail).ring[ring_idx]) };

        // 2. Read the actual descriptor
        // Note: In a robust implementation, we would follow the 'next' chain.
        // For Console, buffers are usually single descriptors.
        let desc_ptr = unsafe { self.desc.add(desc_idx as usize) };
        let addr = unsafe { read_volatile(&(*desc_ptr).addr) } as *mut u8;
        let len = unsafe { read_volatile(&(*desc_ptr).len) };

        // 3. Increment our internal index (but don't update Used yet)
        self.last_avail_idx = self.last_avail_idx.wrapping_add(1);

        Some((desc_idx, addr, len))
    }

    /// Returns a processed buffer to the Used ring.
    /// This tells Linux we are done with it (RX: data read, TX: data written).
    pub fn add_used_buf(&mut self, desc_idx: u16, len_written: u32) {
        let used_idx_val = unsafe { read_volatile(&(*self.used).idx) };
        let ring_idx = (used_idx_val % self.num) as usize;

        let elem = unsafe { &mut (*self.used).ring[ring_idx] };
        unsafe {
            write_volatile(&mut elem.id, desc_idx as u32);
            write_volatile(&mut elem.len, len_written);
        }

        // Memory barrier to ensure ring update is visible before index update
        fence(Ordering::SeqCst);

        unsafe {
            write_volatile(&mut (*self.used).idx, used_idx_val.wrapping_add(1));
        }
    }
}
