extern crate i2cdev;
extern crate i2c_pca9685;

#[cfg(any(target_os = "linux", target_os = "android"))]
use i2cdev::linux::*;
use i2c_pca9685::PCA9685;
use std::{thread, time};

const DEFAULT_PCA9685_ADDRESS: u16 = 0x40;
const SERVO_MAX: u16 = 600;
const SERVO_MIN: u16 = 150;


fn main() {

    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", DEFAULT_PCA9685_ADDRESS).unwrap();
    let mut servos = PCA9685::new(i2cdevice).unwrap();
    servos.set_pwm_freq(60.0).unwrap();
    let _ = servos.reset_all_servos();

    for i in 0..5 {
        println!("Servo number up  {}", i);

        servos.set_pwm(i, 0, SERVO_MIN).unwrap();
        thread::sleep(time::Duration::from_millis(500));
        servos.set_pwm(i, 0, SERVO_MAX).unwrap();

        for j in SERVO_MIN..SERVO_MAX {
            thread::sleep(time::Duration::from_millis(10));
            servos.set_pwm(i, 0, j).unwrap();
        }

        thread::sleep(time::Duration::from_millis(5));
        println!("Servo number down  {}", i);
        thread::sleep(time::Duration::from_millis(500));

        for k in (SERVO_MIN..SERVO_MAX).rev() {
            println!("number is {}", k);
            thread::sleep(time::Duration::from_millis(10));
            servos.set_pwm(i, 0, k).unwrap();
        }

        thread::sleep(time::Duration::from_millis(5));
    }

}
