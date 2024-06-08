use std::thread::sleep;
use std::time::Duration;

use ch347_hal::windows::{CH347, CH347IIC};
use embedded_hal::i2c::{I2c, Operation};

fn main() {
    println!("Demo write read eeprom");
    let mut device = CH347::new(0);
    match device {
        Ok(x) => {
            let mut i2c = CH347IIC::new(&x).unwrap();
            let write_buf: [u8; 6] = [0, 1, 2, 3, 4, 5];
            i2c.write(0x50, &write_buf).unwrap();

            sleep(Duration::from_millis(100));
            let address: [u8; 1] = [0];
            let mut buffer = [0; 5];
            i2c.write_read(0x50, &address, &mut buffer).unwrap();
            println!("{:?}", buffer);
        }
        Err(message) => print!("{}", message),
    }
}
