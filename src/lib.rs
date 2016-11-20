//! PWM 16 channel controller i2cbus
//! Data sheet https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf
//!

extern crate i2cdev;
extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};
use i2cdev::core::{I2CDevice, I2CError};
use std::{thread, time};

const Default_PCA9685_ADDRESS: u8 = 0x40;
const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
const SUBADR1: u8 = 0x02;
const SUBADR2: u8 = 0x03;
const SUBADR3: u8 = 0x04;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_LED_ON_L: u8 = 0xFA;
const ALL_LED_ON_H: u8 = 0xFB;
const ALL_LED_OFF_L: u8 = 0xFC;
const ALL_LED_OFF_H: u8 = 0xFD;
const RESTART: u8 = 0x80;
const SLEEP: u8 = 0x10;
const ALLCALL: u8 = 0x01;
const INVRT: u8 = 0x10;
const OUTDRV: u8 = 0x04;
const SWRESET: u8 = 0x06;
const ALLDEV: u8 = 0x00;

fn sleep_5ms() {
    let five_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(five_millis);
}

pub struct PCA9685<T: I2CDevice + Sized> {
    i2cdev: T,
}

impl<T> PCA9685<T>
    where T: I2CDevice + Sized
{
    pub fn new(mut i2cdev: T) -> Result<PCA9685<T>, T::I2CError> {
        // self.set_all_pwm(0, 0)
        i2cdev.smbus_write_byte_data(MODE2, OUTDRV)?;
        i2cdev.smbus_write_byte_data(MODE1, ALLCALL)?;
        sleep_5ms();  // wait for oscillator
        let mode1 = i2cdev.smbus_read_byte_data(MODE1);
        mode1 = (mode1)? & SLEEP;  // wake up
        i2cdev.smbus_write_byte_data(MODE1, mode1)?;
        sleep_5ms(); // wait for oscillator
        Ok(PCA9685 { i2cdev: i2cdev })
    }

    pub fn set_pwm_freq(self, freq: f32) -> Result<(), T::I2CError> {
        // Set the PWM frequency to the provided value in hertz.
        let prescaleval = 25000000.0; // 25MHz
        prescaleval /= 4096.0; // 12-bit
        prescaleval /= freq;
        prescaleval -= 1.0;
        let prescale: u32 = prescaleval + 0.5;
        let oldmode = self._device.readU8(MODE1);
        let newmode = (oldmode & 0x7F) | SLEEP;    // sleep
        self.i2cdev.smbus_write_byte_data(MODE1, newmode);  // go to sleep
        self.i2cdev.smbus_write_byte_data(PRESCALE, prescale);
        self.i2cdev.smbus_write_byte_data(MODE1, oldmode);
        sleep_5ms();
        self.i2cdev.smbus_write_byte_data(MODE1, oldmode | RESTART);
        Ok(())
    }

    // def set_pwm_freq(self, freq_hz):
    //
    // def set_pwm(self, channel, on, off):
    // """Sets a single PWM channel."""
    // self._device.write8(LED0_ON_L+4*channel, on & 0xFF)
    // self._device.write8(LED0_ON_H+4*channel, on >> 8)
    // self._device.write8(LED0_OFF_L+4*channel, off & 0xFF)
    // self._device.write8(LED0_OFF_H+4*channel, off >> 8)
    //

    // def set_all_pwm(self, on, off):
    // """Sets all PWM channels."""
    // self._device.write8(ALL_LED_ON_L, on & 0xFF)
    // self._device.write8(ALL_LED_ON_H, on >> 8)
    // self._device.write8(ALL_LED_OFF_L, off & 0xFF)
    // self._device.write8(ALL_LED_OFF_H, off >> 8)
    // def software_reset(i2c=None, **kwargs):

    pub fn reset_all_servos(&self) {
        self.i2cdev.smbus_write_byte_data(0x00);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
