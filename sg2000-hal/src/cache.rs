use riscv::asm::fence;
use xuantie_riscv::asm::dcache_cpa;

/// # Safety
///
/// WOij
#[inline(always)]
pub unsafe fn dcache_flush(start_addr: usize, len: usize) {
    let mut addr = start_addr & !63;
    let end = start_addr + len;

    while addr < end {
        unsafe { dcache_cpa(addr) };
        addr += 64;
    }

    fence();
}
