use embedded_hal::{i2c, spi};
use std::ffi::*;

#[repr(C)]
#[repr(packed)]
pub struct SpiConfig {
    mode: c_uchar,                     // 0-3:SPI Mode0/1/2/3
    clock: c_uchar, // 0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz，7=468.75KHz
    byte_order: c_uchar, // 0=低位在前(LSB), 1=高位在前(MSB)
    spi_write_read_interval: c_ushort, // SPI接口常规读取写入数据命令，单位为uS
    spi_out_default_data: c_uchar, // SPI读数据时默认输出数据
    chip_select: c_ulong, // 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效: 位1位0为00/01分别选择CS1/CS2引脚作为低电平有效片选
    cs0_polarity: c_uchar, // 位0：片选CS1极性控制：0：低电平有效；1：高电平有效；
    cs1_polarity: c_uchar, // 位0：片选CS2极性控制：0：低电平有效；1：高电平有效；
    is_auto_deative_cs: c_ushort, // 操作完成后是否自动撤消片选
    active_delay: c_ushort, // 设置片选后执行读写操作的延时时间,单位us
    delay_deactive: c_ulong, // 撤消片选后执行读写操作的延时时间,单位us
}
#[repr(C)]
#[allow(non_camel_case_types)]
enum EEPROM_TYPE {
    ID_24C01 = 0x01,
    ID_24C02,
    ID_24C04,
    ID_24C08,
    ID_24C16,
    ID_24C32,
    ID_24C64,
    ID_24C128,
    ID_24C256,
    ID_24C512,
    ID_24C1024,
    ID_24C2048,
    ID_24C4096,
}

extern "C" {
    // open/close
    // 打开USB设备
    fn CH347OpenDevice(iIndex: c_ulong) -> c_int;
    // 关闭USB设备
    fn CH347CloseDevice(iIndex: c_ulong) -> c_int; // return bool

    /***************SPI********************/
    // SPI控制器初始化
    fn CH347SPI_Init(iIndex: c_ulong, config: *const SpiConfig) -> c_int;
    fn CH347SPI_ChangeCS(
        iIndex: c_ulong, // 指定设备序号
        iStatus: c_uchar,
    ) -> c_int;
    pub fn CH347SPI_SetChipSelect(
        iIndex: c_ulong,           // 指定设备序号
        iEnableSelect: c_short,    // 低八位为CS1，高八位为CS2; 字节值为1=设置CS,为0=忽略此CS设置
        iChipSelect: c_short,      // 低八位为CS1，高八位为CS2;片选输出,0=撤消片选,1=设置片选
        iIsAutoDeativeCS: c_ulong, // 低16位为CS1，高16位为CS2;操作完成后是否自动撤消片选
        iActiveDelay: c_ulong, // 低16位为CS1，高16位为CS2;设置片选后执行读写操作的延时时间,单位us
        iDelayDeactive: c_ulong, // 低16位为CS1，高16位为CS2;撤消片选后执行读写操作的延时时间:c_ulong,单位us
    ) -> c_int;

    // 处理SPI数据流,4线接口
    fn CH347StreamSPI4(
        index: c_ulong,        // 指定设备序号
        chipSelect: c_ulong,   // 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效
        Length: c_ulong,       // 准备传输的数据字节数
        ioBuffer: *mut c_void, // 指向一个缓冲区,放置准备从DOUT写出的数据,返回后是从DIN读入的数据
    ) -> c_int;

    /********IIC***********/
    // 设置串口流模式
    fn CH347I2C_Set(
        iIndex: c_ulong, // 指定设备序号
        iMode: c_ulong,  // 指定模式,见下行
                         // 位1-位0: I2C接口速度/SCL频率, 00=低速/20KHz,01=标准/100KHz(默认值),10=快速/400KHz,11=高速/750KHz
                         // 其它保留,必须为0
    ) -> c_int;

    // 设置硬件异步延时,调用后很快返回,而在下一个流操作之前延时指定毫秒数
    fn CH347I2C_SetDelaymS(
        iIndex: c_ulong, // 指定设备序号
        iDelay: c_ulong, // 指定延时的毫秒数
    ) -> c_int;

    // 处理I2C数据流,2线接口,时钟线为SCL引脚,数据线为SDA引脚
    fn CH347StreamI2C(
        iIndex: c_ulong,             // 指定设备序号
        iWriteLength: c_ulong,       // 准备写出的数据字节数
        iWriteBuffer: *const c_void, // 指向一个缓冲区,放置准备写出的数据,首字节通常是I2C设备地址及读写方向位
        iReadLength: c_ulong,        // 准备读取的数据字节数
        oReadBuffer: *const c_void,  // 指向一个缓冲区,返回后是读入的数据
    ) -> c_int;
}

pub struct CH347 {
    index: u32,
}
pub struct CH347SPI {
    index: u32,
}
pub struct CH347IIC {
    index: u32,
}

impl CH347 {
    pub fn new(index: u32) -> Result<Self, String> {
        let ret = unsafe { CH347OpenDevice(index) };
        if ret >= 0 {
            Ok(CH347 { index })
        } else {
            Err(format!("device open failed, ret:{}", ret))
        }
    }
}
impl Drop for CH347 {
    fn drop(&mut self) {
        unsafe {
            CH347CloseDevice(self.index);
        }
    }
}
impl CH347IIC {
    pub fn new(device: &CH347) -> Result<Self, String> {
        let ret = unsafe { CH347I2C_Set(device.index, 0x01) };
        if ret == 0 {
            return Err("iic init failed".into());
        }
        Ok(Self {
            index: device.index,
        })
    }
}
impl CH347SPI {
    pub fn new(device: &CH347) -> Result<Self, String> {
        let spi_config = SpiConfig {
            mode: 3,                    // 0-3:SPI Mode0/1/2/3
            clock: 5, // 0=60MHz, 1=30MHz, 2=15MHz, 3=7.5MHz, 4=3.75MHz, 5=1.875MHz, 6=937.5KHz，7=468.75KHz
            byte_order: 1, // 0=低位在前(LSB), 1=高位在前(MSB)
            spi_write_read_interval: 0, // SPI接口常规读取写入数据命令，单位为uS
            spi_out_default_data: 0xff, // SPI读数据时默认输出数据
            chip_select: 0x80, // 片选控制, 位7为0则忽略片选控制, 位7为1则参数有效: 位1位0为00/01分别选择CS1/CS2引脚作为低电平有效片选
            cs0_polarity: 0,   // 位0：片选CS1极性控制：0：低电平有效；1：高电平有效；
            cs1_polarity: 0,   // 位0：片选CS2极性控制：0：低电平有效；1：高电平有效；
            is_auto_deative_cs: 0, // 操作完成后是否自动撤消片选
            active_delay: 0,   // 设置片选后执行读写操作的延时时间,单位us
            delay_deactive: 0, // 撤消片选后执行读写操作的延时时间,单位us
        };
        let ret = unsafe { CH347SPI_Init(device.index, (&spi_config) as *const SpiConfig) };
        if ret == 0 {
            return Err("spi init failed".into());
        }
        Ok(Self {
            index: device.index,
        })
    }
}

impl i2c::ErrorType for CH347IIC {
    type Error = i2c::ErrorKind;
}

impl i2c::I2c for CH347IIC {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let address = address << 1;
        if operations.len() == 2 {
            let mut writelenth = 1;
            let mut writebuffer = vec![address];
            for operation in operations.iter_mut() {
                if let i2c::Operation::Write(x) = operation {
                    writebuffer.extend_from_slice(x);
                    writelenth += x.len();
                }
                if let i2c::Operation::Read(x) = operation {
                    unsafe {
                        let ret = CH347StreamI2C(
                            self.index,
                            writelenth as u32,
                            writebuffer.as_slice().as_ptr() as *mut c_void,
                            x.len() as u32,
                            x.as_ptr() as *mut c_void,
                        );
                        if ret == 0 {
                            return Err(i2c::ErrorKind::Bus);
                        }
                    }
                }
            }
        } else {
            assert!(operations.len() == 1);
            let mut writelenth = 1;
            let mut writebuffer = vec![address];
            for operation in operations.iter_mut() {
                if let i2c::Operation::Write(x) = operation {
                    writebuffer.extend_from_slice(x);
                    writelenth += x.len();
                    let ret = unsafe {
                        CH347StreamI2C(
                            self.index,
                            writelenth as u32,
                            writebuffer.as_slice().as_ptr() as *mut c_void,
                            0 as u32,
                            0 as *mut c_void,
                        )
                    };
                    if ret == 0 {
                        return Err(i2c::ErrorKind::Bus);
                    }
                }
                if let i2c::Operation::Read(x) = operation {
                    let ret = unsafe {
                        CH347StreamI2C(
                            self.index,
                            writelenth as u32,
                            writebuffer.as_slice().as_ptr() as *mut c_void,
                            x.len() as u32,
                            x.as_ptr() as *mut c_void,
                        )
                    };
                    if ret == 0 {
                        return Err(i2c::ErrorKind::Bus);
                    }
                }
            }
        }
        Ok(())
    }
}

impl spi::ErrorType for CH347SPI {
    type Error = spi::ErrorKind;
}
impl spi::SpiDevice for CH347SPI {
    fn transaction(
        &mut self,
        operations: &mut [spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        unsafe { CH347SPI_ChangeCS(self.index, 0) };
        operations.iter_mut().for_each(|op| match op {
            spi::Operation::Read(read_buffer) => {
                read_buffer.iter_mut().for_each(|x| *x = 0xff);
                unsafe {
                    CH347StreamSPI4(
                        self.index,
                        0,
                        read_buffer.len() as u32,
                        read_buffer.as_ptr() as *mut c_void,
                    );
                }
            }
            spi::Operation::Write(x) => unsafe {
                CH347StreamSPI4(self.index, 0, x.len() as u32, x.as_ptr() as *mut c_void);
            },
            spi::Operation::TransferInPlace(x) => unsafe {
                CH347StreamSPI4(self.index, 0, x.len() as u32, x.as_ptr() as *mut c_void);
            },
            spi::Operation::Transfer(read_buffer, write_buffer) => {
                assert!(read_buffer.len() == write_buffer.len());
                read_buffer.copy_from_slice(write_buffer);
                unsafe {
                    CH347StreamSPI4(
                        self.index,
                        0,
                        read_buffer.len() as u32,
                        read_buffer.as_ptr() as *mut c_void,
                    );
                }
            }
            _ => (),
        });
        unsafe { CH347SPI_ChangeCS(self.index, 1) };
        Ok(())
    }
}
