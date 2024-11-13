use std::marker::PhantomData;

use config::*;
use embedded_hal::digital::Error as PinError;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use embedded_hal_async::i2c::{ErrorKind, I2c};
use settings::*;

mod config;
mod settings;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E1, E2> {
    /// An error occurred while communicating with the camera over I2C. The inner error contains the specific error.
    I2c(E1),
    /// Failed to toggle pin
    Pin(E2),
}

impl From<ErrorKind> for Error<ErrorKind, embedded_hal::digital::ErrorKind> {
    fn from(error: ErrorKind) -> Self {
        Error::I2c(error)
    }
}

impl From<embedded_hal::digital::ErrorKind> for Error<ErrorKind, embedded_hal::digital::ErrorKind> {
    fn from(error: embedded_hal::digital::ErrorKind) -> Self {
        Error::Pin(error)
    }
}

/// OV2640 driver
pub struct OV2640<I2C, P, D> {
    i2c: I2C,
    addr: u8,
    rst: P,
    delay: D,
}

impl<I2C, P, D, E1, E2> OV2640<I2C, P, D>
where
    I2C: I2c<Error = E1>,
    P: OutputPin<Error = E2>,
    D: DelayNs,
    Error<E1, E2>: From<E1>, // Allow conversion from I2C error
    Error<E1, E2>: From<E2>, // Allow conversion from Pin error
{
    pub fn new(i2c: I2C, rst: P, delay: D) -> Self {
        OV2640 {
            i2c,
            addr: 0x60,
            rst,
            delay,
        }
    }

    /// Initialize the camera
    pub async fn init(&mut self) -> Result<(), Error<E1, E2>> {
        self.rst.set_low()?;
        self.delay.delay_ns(DELAYS.POST_RESET_DELAY);
        self.rst.set_high()?;
        self.delay.delay_ns(DELAYS.POST_RESET_DELAY);

        // Software reset
        self.write_reg(0xff, 0x01).await?;
        self.write_reg(0x12, 0x80).await?;

        self.delay.delay_ns(DELAYS.POST_SW_RESET_DELAY);

        // Initialize with JPEG settings
        for cfg in JPEG_INIT {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        // Switch to YUV422 mode
        for cfg in YUV422 {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        // Enable JPEG mode
        for cfg in JPEG {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set camera resolution
    pub async fn set_resolution(&mut self, resolution: Resolution) -> Result<(), Error<E1, E2>> {
        // Select appropriate configuration based on resolution
        let config = match resolution {
            Resolution::_160x120 => RESOLUTION_160X120,
            Resolution::_320x240 => RESOLUTION_320X240,
            Resolution::_640x480 => RESOLUTION_640X480,
            Resolution::_800x600 => RESOLUTION_800X600,
            Resolution::_1024x768 => RESOLUTION_1024X768,
            Resolution::_1280x960 => RESOLUTION_1280X960,
        };

        // Apply configuration
        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set special effect
    pub async fn set_special_effect(&mut self, effect: SpecialEffect) -> Result<(), Error<E1, E2>> {
        let config = match effect {
            SpecialEffect::Normal => SPECIAL_EFFECT_NORMAL,
            SpecialEffect::Antique => SPECIAL_EFFECT_ANTIQUE,
            SpecialEffect::Bluish => SPECIAL_EFFECT_BLUISH,
            SpecialEffect::Greenish => SPECIAL_EFFECT_GREENISH,
            SpecialEffect::Reddish => SPECIAL_EFFECT_REDDISH,
            SpecialEffect::BlackWhite => SPECIAL_EFFECT_BW,
            SpecialEffect::Negative => SPECIAL_EFFECT_NEGATIVE,
            SpecialEffect::BlackWhiteNegative => SPECIAL_EFFECT_BW_NEGATIVE,
        };

        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set light mode (white balance)
    pub async fn set_light_mode(&mut self, mode: LightMode) -> Result<(), Error<E1, E2>> {
        let config = match mode {
            LightMode::Auto => LIGHT_MODE_AUTO,
            LightMode::Sunny => LIGHT_MODE_SUNNY,
            LightMode::Cloudy => LIGHT_MODE_CLOUDY,
            LightMode::Office => LIGHT_MODE_OFFICE,
            LightMode::Home => LIGHT_MODE_HOME,
        };

        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set brightness level
    pub async fn set_brightness(&mut self, level: Level) -> Result<(), Error<E1, E2>> {
        let config = match level {
            Level::VeryLow => BRIGHTNESS_M2,
            Level::Low => BRIGHTNESS_M1,
            Level::Normal => BRIGHTNESS_0,
            Level::High => BRIGHTNESS_P1,
            Level::VeryHigh => BRIGHTNESS_P2,
        };

        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set saturation level
    pub async fn set_saturation(&mut self, level: Level) -> Result<(), Error<E1, E2>> {
        let config = match level {
            Level::VeryLow => SATURATION_M2,
            Level::Low => SATURATION_M1,
            Level::Normal => SATURATION_0,
            Level::High => SATURATION_P1,
            Level::VeryHigh => SATURATION_P2,
        };

        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Set contrast level
    pub async fn set_contrast(&mut self, level: Level) -> Result<(), Error<E1, E2>> {
        let config = match level {
            Level::VeryLow => CONTRAST_M2,
            Level::Low => CONTRAST_M1,
            Level::Normal => CONTRAST_0,
            Level::High => CONTRAST_P1,
            Level::VeryHigh => CONTRAST_P2,
        };

        for cfg in config {
            self.write_reg(cfg.reg, cfg.val).await?;
        }

        Ok(())
    }

    /// Write to a camera register
    async fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Error<E1, E2>> {
        let buf = [reg, val];
        self.i2c.write(self.addr, &buf).await?;
        self.delay.delay_ns(DELAYS.WRITE_DELAY);

        Ok(())
    }

    /// Read from a camera register
    async fn read_reg(&mut self, reg: u8) -> Result<u8, Error<E1, E2>> {
        let mut buf = [0u8];
        self.i2c.write(self.addr, &[reg]).await?;
        self.i2c.read(self.addr | 1, &mut buf).await?;
        Ok(buf[0])
    }

    /// Release I2C peripheral
    pub fn release(self) -> I2C {
        self.i2c
    }
}
