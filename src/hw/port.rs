



pub struct Port {
    num: u16
}

impl Port {
    #[inline]
    pub fn new(num: u16) -> Self {
        Self { num }
    }

    #[inline]
    pub unsafe fn read(&self) -> u8 {
        let mut res: u8;
        core::arch::asm!(
            "in al, dx",
            out("al") res,
            in("dx") self.num,
        );
        res
    }
    #[inline]
    pub unsafe fn write(&mut self, data: u8) {
        core::arch::asm!(
            "out dx, al",
            in("dx") self.num,
            in("al") data,
        )
    }
}
