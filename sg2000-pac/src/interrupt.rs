pub use riscv::interrupt::Exception;
pub use riscv::interrupt::Interrupt as CoreInterrupt;
pub use riscv::{
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
    interrupt::{disable, enable, free, nested},
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "30 - UART0"]
    UART0 = 30,
    #[doc = "31 - UART1"]
    UART1 = 31,
    #[doc = "41 - GPIO0"]
    GPIO0 = 41,
    #[doc = "42 - GPIO1"]
    GPIO1 = 42,
    #[doc = "43 - GPIO2"]
    GPIO2 = 43,
    #[doc = "44 - GPIO3"]
    GPIO3 = 44,
    #[doc = "61 - MBOX"]
    MBOX = 61,
}
