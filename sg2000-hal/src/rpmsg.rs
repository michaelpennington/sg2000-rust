use core::ptr::{read_volatile, write_volatile};

use crate::{
    mailbox::Mailbox,
    resource_table::{RESOURCE_TABLE, VRING_ALIGN, VRING_NUM},
    virtio::VirtQueue,
};

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
            flags: 0,
        }
    }
}

pub const RPMSG_ADDR_NS: u32 = 53;
pub const RPMSG_RESERVED: u32 = 0;

pub struct RpmsgDevice<'a> {
    tx_mailbox: Mailbox<'a>,
    rx_mailbox: Mailbox<'a>,
    tx_queue: VirtQueue,
    rx_queue: VirtQueue,
}

impl<'a> RpmsgDevice<'a> {
    /// # Safety
    /// This function creates VirtQueues from raw memory addresses defined in the
    /// Resource Table. Ensure no other code is managing these rings.
    ///
    /// It assumes:
    /// * VRing 0 @ 0x8F528000 (TX for Device / RX for Host)
    /// * VRing 1 @ 0x8F52C000 (RX for Device / TX for Host)
    pub unsafe fn new(tx_mailbox: Mailbox<'a>, rx_mailbox: Mailbox<'a>) -> Self {
        let tx_queue = unsafe {
            VirtQueue::from_resource_table_addr(0x8F528000, VRING_NUM as u16, VRING_ALIGN)
        };
        let rx_queue = unsafe {
            VirtQueue::from_resource_table_addr(0x8F52C000, VRING_NUM as u16, VRING_ALIGN)
        };
        Self {
            tx_mailbox,
            rx_mailbox,
            tx_queue,
            rx_queue,
        }
    }

    pub fn is_driver_ok(&self) -> bool {
        (unsafe { read_volatile(&RESOURCE_TABLE.rpmsg_vdev.status) } & 0x4) != 0
    }

    pub fn announce(&mut self, name: &str, src_addr: u32) -> Result<(), &'static str> {
        let payload = RpmsgNsMsg::new(name, src_addr);

        self.send_raw(
            src_addr,
            RPMSG_ADDR_NS,
            size_of::<RpmsgNsMsg>() as u16,
            |buf| {
                let ptr = buf.as_mut_ptr() as *mut RpmsgNsMsg;
                unsafe { write_volatile(ptr, payload) };
            },
        )
    }

    pub fn send(&mut self, src: u32, dst: u32, data: &[u8]) -> Result<(), &'static str> {
        self.send_raw(src, dst, data.len() as u16, |buf| {
            buf.copy_from_slice(data);
        })
    }

    fn send_raw<F>(&mut self, src: u32, dst: u32, len: u16, writer: F) -> Result<(), &'static str>
    where
        F: FnOnce(&mut [u8]),
    {
        let mut tx_desc_idx = None;
        for _ in 0..1000 {
            tx_desc_idx = self.tx_queue.get_avail_buf();
            if tx_desc_idx.is_some() {
                break;
            }
        }
        let desc_idx = tx_desc_idx.ok_or("TX Ring Full")?;

        let buffer = unsafe { self.tx_queue.get_buf_slice(desc_idx) };

        let header = RpmsgHeader {
            src,
            dst,
            reserved: 0,
            len,
            flags: 0,
        };

        let head_ptr = buffer.as_mut_ptr() as *mut RpmsgHeader;
        unsafe {
            write_volatile(head_ptr, header);
        }
        let payload_slice = unsafe {
            core::slice::from_raw_parts_mut(
                buffer.as_mut_ptr().add(size_of::<RpmsgHeader>()),
                len as usize,
            )
        };
        writer(payload_slice);

        let total_len = size_of::<RpmsgHeader>() + len as usize;
        self.tx_queue.add_used_buf(desc_idx, total_len as u32);

        if !self.tx_mailbox.send_data(0) {
            return Err("Mailbox Send Failed");
        }

        Ok(())
    }

    pub fn poll<F>(&mut self, mut callback: F) -> bool
    where
        F: FnMut(u32, u32, &[u8]),
    {
        if self.rx_mailbox.is_pending() {
            let _ = self.rx_mailbox.read_data();
        }

        if let Some(desc_idx) = self.rx_queue.get_avail_buf() {
            let buf = unsafe { self.rx_queue.get_buf_slice(desc_idx) };

            if buf.len() >= size_of::<RpmsgHeader>() {
                let header = unsafe { &*(buf.as_ptr() as *const RpmsgHeader) };
                let payload_len = header.len as usize;

                if buf.len() >= size_of::<RpmsgHeader>() + payload_len {
                    let data_start = size_of::<RpmsgHeader>();
                    let data = &buf[data_start..data_start + payload_len];

                    callback(header.src, header.dst, data);
                }
            }

            self.rx_queue.add_used_buf(desc_idx, 0);

            let _ = self.tx_mailbox.send_data(1);

            return true;
        }

        false
    }
}
