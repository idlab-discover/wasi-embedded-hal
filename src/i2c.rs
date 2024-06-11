//! Implementation of [`embedded-hal`] I2C traits
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal

/// Macro that adds HAL 1.0 support for a given I2C bindings.
#[cfg(feature = "hal_1_0")]
#[macro_export]
macro_rules! add_i2c_hal {
    ($i2c:ident) => {
        #[cfg(feature = "use_alloc")]
        use alloc::vec::Vec;

        use embedded_hal::i2c::{
            NoAcknowledgeSource as HalNoAcknowledgeSource, Operation as HalOperation,
        };
        use $i2c::{ErrorCode, I2c, NoAcknowledgeSource, Operation};

        #[derive(Debug)]
        pub struct I2CError {
            err: ErrorCode,
        }

        impl embedded_hal::i2c::Error for I2CError {
            fn kind(&self) -> embedded_hal::i2c::ErrorKind {
                use embedded_hal::i2c::ErrorKind;

                let errno = self.err;

                match errno {
                    ErrorCode::Bus => ErrorKind::Bus,
                    ErrorCode::ArbitrationLoss => ErrorKind::ArbitrationLoss,
                    ErrorCode::NoAcknowledge(sour) => match sour {
                        NoAcknowledgeSource::Address => {
                            ErrorKind::NoAcknowledge(HalNoAcknowledgeSource::Address)
                        }
                        NoAcknowledgeSource::Data => {
                            ErrorKind::NoAcknowledge(HalNoAcknowledgeSource::Data)
                        }
                        NoAcknowledgeSource::Unknown => {
                            ErrorKind::NoAcknowledge(HalNoAcknowledgeSource::Unknown)
                        }
                    },
                    ErrorCode::Overrun => ErrorKind::Overrun,
                    ErrorCode::Other => ErrorKind::Other,
                }
            }
        }

        impl From<I2CError> for ErrorCode {
            fn from(value: I2CError) -> Self {
                value.err
            }
        }

        impl embedded_hal::i2c::ErrorType for I2c {
            type Error = I2CError;
        }

        impl embedded_hal::i2c::I2c for I2c {
            fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
                let data = Self::read(&self, u16::from(address), read.len().try_into().unwrap())
                    .map_err(|e| I2CError { err: e })?;

                for (read, data) in read.iter_mut().zip(data) {
                    *read = data;
                }

                Ok(())
            }

            fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
                Self::write(&self, u16::from(address), write).map_err(|e| I2CError { err: e })
            }

            fn write_read(
                &mut self,
                address: u8,
                write: &[u8],
                read: &mut [u8],
            ) -> Result<(), Self::Error> {
                let data = Self::write_read(
                    &self,
                    u16::from(address),
                    write,
                    read.len().try_into().unwrap(),
                )
                .map_err(|e| I2CError { err: e })?;

                for (read, data) in read.iter_mut().zip(data) {
                    *read = data;
                }

                Ok(())
            }

            fn transaction(
                &mut self,
                address: u8,
                operations: &mut [HalOperation<'_>],
            ) -> Result<(), Self::Error> {
                let opers = operations
                    .iter_mut()
                    .map(|a| match a {
                        HalOperation::Read(r) => Operation::Read(r.len().try_into().unwrap()),
                        HalOperation::Write(w) => Operation::Write(w.to_vec()),
                    })
                    .collect::<Vec<Operation>>();

                let _ = Self::transaction(self, u16::from(address), &opers);

                Ok(())
            }
        }
    };
}

/// Macro that adds HAL 0.2 support for a given I2C bindings.
#[cfg(feature = "hal_0_2")]
#[macro_export]
macro_rules! add_i2c_hal {
    ($i2c:ident) => {
        use embedded_hal::blocking::i2c::Operation as HalOperation;
        use $i2c::{ErrorCode, I2c, NoAcknowledgeSource, Operation};

        #[derive(Debug)]
        pub struct I2CError {
            err: ErrorCode,
        }

        impl embedded_hal::blocking::i2c::Read for I2c {
            type Error = I2CError;

            fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
                let data = Self::read(&self, u16::from(address), read.len().try_into().unwrap())
                    .map_err(|e| I2CError { err: e })?;

                for (read, data) in read.iter_mut().zip(data) {
                    *read = data;
                }

                Ok(())
            }
        }

        impl embedded_hal::blocking::i2c::Write for I2c {
            type Error = I2CError;

            fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
                Self::write(&self, u16::from(address), write).map_err(|e| I2CError { err: e })
            }
        }

        impl embedded_hal::blocking::i2c::WriteRead for I2c {
            type Error = I2CError;

            fn write_read(
                &mut self,
                address: u8,
                write: &[u8],
                read: &mut [u8],
            ) -> Result<(), Self::Error> {
                let data = Self::write_read(
                    &self,
                    u16::from(address),
                    write,
                    read.len().try_into().unwrap(),
                )
                .map_err(|e| I2CError { err: e })?;

                for (read, data) in read.iter_mut().zip(data) {
                    *read = data;
                }

                Ok(())
            }
        }

        impl embedded_hal::blocking::i2c::Transactional for I2c {
            type Error = I2CError;

            fn exec<'a>(
                &mut self,
                address: u8,
                operations: &mut [HalOperation<'a>],
            ) -> Result<(), Self::Error> {
                let opers = operations
                    .iter_mut()
                    .map(|a| match a {
                        HalOperation::Read(r) => Operation::Read(r.len().try_into().unwrap()),
                        HalOperation::Write(w) => Operation::Write(w.to_vec()),
                    })
                    .collect::<Vec<Operation>>();

                let _ = Self::transaction(self, u16::from(address), &opers);

                Ok(())
            }
        }
    };
}
