use core::fmt::Write;

use super::port;



#[repr(u16)]
pub enum Port {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}


pub struct InitedPort {
    p0: port::Port,
    p1: port::Port,
    p2: port::Port,
    p3: port::Port,
    p4: port::Port,
    p5: port::Port,
}

impl Port {
    #[inline]
    pub unsafe fn init(self) -> Option<InitedPort> {
        let num = self as u16;

        let mut p0 = port::Port::new(num + 0);
        let mut p1 = port::Port::new(num + 1);
        let mut p2 = port::Port::new(num + 2);
        let mut p3 = port::Port::new(num + 3);
        let mut p4 = port::Port::new(num + 4);
        let mut p5 = port::Port::new(num + 5);

        p1.write(0x00); // Disable all interrupts
        p3.write(0x80); // Enable DLAB (set baud rate divisor)
        p0.write(0x03); // Set divisor to 3 (lo byte) 38400 baud
        p1.write(0x00); //                  (hi byte)
        p3.write(0x03); // 8 bits, no parity, one stop bit
        p2.write(0xC7); // Enable FIFO, clear them, with 14-byte threshold
        p4.write(0x0B); // IRQs enabled, RTS/DSR set
        p4.write(0x1E); // Set in loopback mode, test the serial chip
        p0.write(0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if p0.read() == 0xAE {
            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            p4.write(0x0F);
            Some(InitedPort { p0, p1, p2, p3, p4, p5 })
        } else {
            None
        }
    }
}

impl InitedPort {
    #[inline]
    pub unsafe fn ready_read(&self) -> bool {
        (self.p5.read() & 1) != 0
    }

    #[inline]
    pub unsafe fn read_sync(&mut self) -> u8 {
        while !self.ready_read() {};
        self.p0.read()
    }

    #[inline]
    pub unsafe fn try_read(&mut self) -> Option<u8> {
        if self.ready_read() {
            Some(self.p0.read())
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn ready_write(&self) -> bool {
        (self.p5.read() & 0x20) != 0
    }

    #[inline]
    pub unsafe fn write_sync(&mut self, data: u8) {
        while !self.ready_write() {};
        self.p0.write(data)
    }

    #[inline]
    pub unsafe fn try_write(&mut self, data: u8) -> bool {
        if self.ready_write() {
            self.p0.write(data);
            true
        } else {
            false
        }
    }
}

impl Write for InitedPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            unsafe { self.write_sync(*b) };
        }
        Ok(())
    }
}
