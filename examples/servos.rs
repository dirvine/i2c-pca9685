extern crate i2cdev;
extern crate i2c_pca9685;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};

const DEFAULT_PCA9685_ADDRESS: u16 = 0x40;
const SERVO_MAX: u8 = 600;
const SERVO_MIN: u8 = 150;


fn main() {

    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();

    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();
    for i in 0..6 {
        servos.set_pwm(i, 0, SERVO_MIN).unwrap();
        thread::sleep(time::Duration::from_millis(500));
        servos.set_pwm(i, 0, SERVO_MAX).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    }

}
