extern crate i2cdev;
extern crate i2c_pca9685;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};

const DEFAULT_PCA9685_ADDRESS: u16 = 0x40;
const SERVO_MIN: u16 = 65;
const SERVO_MAX: u16 = 220;


fn main() {

    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();

    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();
    // servos.reset_all_servos().unwrap();
    for i in 0..4 {
        for j in SERVO_MIN..SERVO_MAX {
            servos.set_pwm(i, 0, j).unwrap();
        }
        thread::sleep(time::Duration::from_millis(500));
        for k in (SERVO_MIN..SERVO_MAX).rev() {
            servos.set_pwm(i, 0, k).unwrap();
        }
        thread::sleep(time::Duration::from_millis(500));
    }
    // servos.reset_all_servos().unwrap();

}
