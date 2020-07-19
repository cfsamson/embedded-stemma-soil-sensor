//! Library for retrieving readings from Adafruit STEMMA Soil Sensor.
//!
//! The implementation is based on the [Adafruit CircuitPython Seesaw library](https://github.com/adafruit/Adafruit_CircuitPython_seesaw).
//!
//! The library is tested and used on a Raspberry Pi 3 B+ board, running Raspbian but uses interfaces
//! compatible with `embedded_hal` so it should work in `no_std` environments as well.
//!
//! ## Example
//!
//! ```rust, ignore
//! pub fn main(interval_ms: u64) {
//!    use stemma_soil_sensor::SoilSensor;
//!    use linux_embedded_hal::Delay;
//!    use embedded_hal::blocking::delay::DelayMs;
//!
//!    let delay = Delay {};
//!    let mut sensor = SoilSensor::init(delay).unwrap();
//!
//!     loop {
//!        let temp = sensor.get_temp().unwrap();
//!        let cap = sensor.get_capacitance().unwrap();
//!        println!("The temperature is: {:.02}", temp);
//!        println!("The capactiance is: {}", cap);
//!        let mut delay = Delay {};
//!        delay.delay_ms(2000u32);
//!    }
//!}
//! ```
//!
//! ## Debugging
//!
//! There are a lot of `debug!` information in the code which will be available on debug builds.
//! Attaching a logger and setting `RUST_LOG=debug` will yield a lot of information.
use embedded_hal::blocking::delay::DelayUs;
use rppal::i2c::{self, Error as I2CError};
#[macro_use]
extern crate log;
use i2c::I2c;
use thiserror::Error;

mod regs;

// Let the chip get some time to process. https://github.com/adafruit/Adafruit_Seesaw/blob/8728936a5d1a0a7bf2887a82adb0828b70556a45/Adafruit_seesaw.cpp#L745
const STD_PROCESSING_DELAY_MICROS: u16 = 125;

const SENSOR_START_ADDR: u16 = 0x36;
const SENSOR_END_ADDR: u16 = 0x39;

pub type Result<T> = std::result::Result<T, SoilSensErr>;

pub struct SoilSensor<D: DelayUs<u16>> {
    channel: i2c::I2c,
    delay: D,
}

impl<D: DelayUs<u16>> SoilSensor<D> {
    pub fn init(mut delay: D) -> Result<Self> {
        let channel = init::channel_init(&mut delay)?;

        Ok(SoilSensor { channel, delay })
    }

    /// Reads the temperature off the soil sensor. The temperature is in Celcius.
    ///
    /// The temperature sensor is not high precision but should be indicate the temperature
    /// +/- 2 degrees.
    pub fn get_temp(&mut self) -> Result<f32> {
        let l_reg = regs::base::SEESAW_STATUS_BASE;
        let h_reg = regs::func::SEESAW_STATUS_TEMP;
        let delay = STD_PROCESSING_DELAY_MICROS;

        let mut buffer = [0u8; 4];
        self.read(l_reg, h_reg, &mut buffer[..], delay)?;
        let tmp_val = i32::from_be_bytes(buffer) as f32;

        // See: https://github.com/adafruit/Adafruit_Seesaw/blob/8728936a5d1a0a7bf2887a82adb0828b70556a45/Adafruit_seesaw.cpp#L664
        let temp_celcius = (1.0 / (1u32 << 16) as f32) * tmp_val;
        Ok(temp_celcius)
    }

    /// Read the value of the moisture sensor
    ///
    /// The values ranges from 200 (very dry) to 2000 (very wet).
    ///
    /// # Errors
    /// This method will try to read the value from the sensors 3 times before
    /// it returns a `SoilSensErr::MoistureReadErr` if no read is successful.
    pub fn get_capacitance(&mut self) -> Result<u16> {
        let l_reg: u8 = regs::base::SEESAW_TOUCH_BASE;
        let h_reg: u8 = regs::touch::SEESAW_TOUCH_CHANNEL_OFFSET;
        let mut buff = [0u8; 2];
        let mut retry_counter = 0;

        while retry_counter < 3 {
            self.delay.delay_us(1000);
            // NB! Setting this to 1000 (like in the C library) errors.
            if let Err(e) = self.read(l_reg, h_reg, &mut buff, 5000) {
                debug!("Error reading capacitance: {}", e);
                retry_counter += 1;
                continue;
            }

            // A read before the chip is ready will be 0xFFFF
            let cap = u16::from_be_bytes(buff);
            if cap < u16::max_value() {
                return Ok(cap);
            }
        }

        Err(SoilSensErr::MoistureReadErr)
    }

    /// Read an arbitrary I2C register range on the device.
    ///
    /// Delay is needed to allow the board to process the request.
    fn read(&mut self, reg_low: u8, reg_high: u8, buff: &mut [u8], delay_us: u16) -> Result<()> {
        self.channel.write(&[reg_low, reg_high])?;
        self.delay.delay_us(delay_us);
        self.channel.read(buff)?;
        debug!("{:?}", buff);
        Ok(())
    }
}

mod init {
    use super::*;

    /// Initialize the channel
    pub fn channel_init<D: DelayUs<u16>>(delay: &mut D) -> Result<I2c> {
        let mut chan = i2c::I2c::new()?;

        for adr in SENSOR_START_ADDR..=SENSOR_END_ADDR {
            debug!("Connecting to adr: {:#X}", adr);

            match try_read_chan(&mut chan, adr, delay) {
                Ok(resp) => {
                    if resp != regs::SEESAW_HW_ID_CODE {
                        return Err(SoilSensErr::from(I2CError::InvalidSlaveAddress(adr)));
                    } else {
                        debug!("HW ID mismatch. Exp: {}, got: {}", resp, adr);
                        return Ok(chan);
                    }
                }
                Err(SoilSensErr::I2C {
                    source: I2CError::InvalidSlaveAddress(adr),
                }) => {
                    debug!("Invalid address: {}", adr);
                    continue;
                }
                Err(e) => {
                    debug!("Unexpected err: {}", e);
                    continue;
                }
            }
        }

        Err(SoilSensErr::HwNotFound)
    }

    // The fallible initalization code which we'll call for the entire valid address range
    fn try_read_chan<D: DelayUs<u16>>(chan: &mut I2c, adr: u16, delay: &mut D) -> Result<u8> {
        let reg_high = regs::base::SEESAW_STATUS_BASE;
        let reg_low = regs::func::SEESAW_STATUS_HW_ID;
        chan.set_slave_address(adr)?;
        chan.write(&[reg_high, reg_low])?;
        let mut buffer = [0];
        delay.delay_us(STD_PROCESSING_DELAY_MICROS);

        let numbytes = chan.read(&mut buffer)?;
        debug!("{} byte(s):", numbytes);
        debug!("{:?}", buffer);
        Ok(buffer[0])
    }
}

#[derive(Debug, Error)]
pub enum SoilSensErr {
    #[error("Couldn't get a valid reading from the moisture sensor.")]
    MoistureReadErr,
    #[error("Couldn't connect to the sensor.")]
    HwNotFound,
    #[error("I2C connection error. {source}")]
    I2C {
        #[from]
        source: i2c::Error,
    },
}