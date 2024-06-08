use ch347_hal::windows::{CH347, CH347IIC};
use embedded_hal::i2c::{I2c, Operation};

fn main() {
    println!("Demo i2cscan");
    let mut device = CH347::new(0);
    match device {
        Ok(x) => {
            for i in 0..128{
                let mut i2c = CH347IIC::new(&x).unwrap();
                let write_buf: [u8;1] = [0];
                let result = i2c.write(i, &write_buf);
                if let Ok(x) = result{
                    println!("find device, addr:0x{:x}",i);
                } 
            }
        }
        Err(message) => print!("{}", message),
    }
}