use sg2000_pac as pac;

static mut TAKEN: bool = false;

macro_rules! impl_peripherals {
    ($($field:ident: $type:ident),* $(,)?) => {
        #[doc = "All the peripherals."]
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(
                #[doc = concat!("Wrapper for `", stringify!($type), "`")]
                pub $field: $type<'static>,
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
            /// # Safety
            ///
            /// por que
            pub unsafe fn steal() -> Self {
                Self {
                    $($field: unsafe { $type::steal() }),*
                }
            }
        }

        $(
            #[doc = concat!("Wrapper for `pac::", stringify!($type), "`")]
            pub struct $type<'a> {
                pub inner: pac::$type,
                _phantom: core::marker::PhantomData<&'a mut ()>,
            }

            impl $type<'_> {
                #[doc = "Steal this peripheral."]
                #[inline]
                /// # Safety
                ///
                /// por que
                pub unsafe fn steal() -> Self {
                    Self {
                        inner: unsafe { pac::$type::steal() },
                        _phantom: core::marker::PhantomData,
                    }
                }
            }

            impl core::ops::Deref for $type<'_> {
                type Target = pac::$type;
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl core::ops::DerefMut for $type<'_> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
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
