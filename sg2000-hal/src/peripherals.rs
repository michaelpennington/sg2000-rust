use sg2000_pac as pac;

static mut TAKEN: bool = false;

macro_rules! impl_peripherals {
    ($($field:ident: $type:ident),* $(,)?) => {
        #[doc = "All the peripherals."]
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(
                #[doc = concat!("Wrapper for `", stringify!($type), "`")]
                pub $field: $type,
            )*
        }

        impl Peripherals {
            #[doc = "Returns all the peripherals *once*."]
            #[inline]
            pub fn take() -> Option<Self> {
                critical_section::with(|_| {
                    if unsafe { TAKEN } {
                        return None;
                    }
                    unsafe { TAKEN = true; }
                    Some(unsafe { Self::steal() })
                })
            }

            #[doc = "Steal all the peripherals."]
            #[inline]
            pub unsafe fn steal() -> Self {
                let p = unsafe { pac::Peripherals::steal() };
                Self {
                    $($field: $type(p.$field)),*
                }
            }
        }

        $(
            #[doc = concat!("Wrapper for `pac::", stringify!($type), "`")]
            pub struct $type(pub pac::$type);

            impl $type {
                #[doc = "Steal this peripheral."]
                #[inline]
                pub unsafe fn steal() -> Self {
                    Self(unsafe { pac::$type::steal() })
                }
            }

            impl core::ops::Deref for $type {
                type Target = pac::$type;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl core::ops::DerefMut for $type {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )*
    }
}

impl_peripherals! {
    plic: Plic,
    clint: Clint,
    gpio0: Gpio0,
    gpio1: Gpio1,
    gpio2: Gpio2,
    gpio3: Gpio3,
    rtcsys_gpio: RtcsysGpio,
    uart0: Uart0,
    uart1: Uart1,
    uart2: Uart2,
    uart3: Uart3,
    uart4: Uart4,
    rtcsys_uart: RtcsysUart,
    mailboxes: Mailboxes,
}
