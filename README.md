# i2c-pca9685

The Rust `i2c-pca9685` is for manipulating the [Adafruit 16-Channel servo driver](https://www.adafruit.com/product/815) and other boards based on the PCA9685.

Example
-------
```rust
extern crate i2cdev;
extern crate i2c_pca9685;

use i2c_pca9685::PCA9685;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

fn main() {
    match move_servo() {
        Ok(()) => println!("it worked!"),
        Err(err) => println!("uhoh: {}", err),
    }
}

fn move_servo() -> Result<(), LinuxI2CError> {
    let i2cdevice = LinuxI2CDevice::new("/dev/i2c-1", 0x40)?;
    let mut servos = PCA9685::new(i2cdevice)?;

    servos.set_pwm_freq(60.0)?;
    servos.set_pwm(0, 0, 500)?;
    servos.set_pwm(1, 0, 600)?;
    Ok(())
}
```
