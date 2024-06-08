use ch347_hal::windows::{CH347, CH347SPI};
use embedded_hal::spi::{Operation, SpiDevice};

fn main() {
    println!("Demo read spi-nor sfdp");
    let device = CH347::new(0);
    match device{
        Ok(mut x)=> {
            let mut spi = CH347SPI::new(&x).unwrap();
            let cmd: [u8;5] = [0x5a,0,0,0,0xff];
            let mut buffer = [0; 4];
            let mut operations = [Operation::Write(&cmd), Operation::Read(&mut buffer)];
            spi.transaction(&mut operations).ok();
            println!("{:?}", buffer);
        }
        Err(message)=>print!("{}", message)
    }
}
