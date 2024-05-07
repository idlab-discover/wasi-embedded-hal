//! Implementation of [`embedded-hal`] DelayNs trait
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

#[cfg(feature = "hal_1_0")]
#[macro_export]
macro_rules! add_delay_hal {
    ($delay: ident) => {
        use $delay::Delay;

        impl embedded_hal::delay::DelayNs for Delay {
            fn delay_ns(&mut self, ns: u32) {
                Self::delay_ns(self, ns);
            }
        }
    };
}
