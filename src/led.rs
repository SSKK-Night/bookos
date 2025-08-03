use core::ptr::{write_volatile};
use crate::vcell::VolatileCell;
use core::ops::Deref;


#[repr(C)]
pub struct PortRegisters {
    pub dir: VolatileCell<u32>,
    pub dirclr: VolatileCell<u32>,
    pub dirset: VolatileCell<u32>,
    pub dirtgl: VolatileCell<u32>,

    pub out: VolatileCell<u32>,
    pub outclr: VolatileCell<u32>,
    pub outset: VolatileCell<u32>,
    pub outtgl: VolatileCell<u32>,

    pub r#in: VolatileCell<u32>,
    pub ctrl: VolatileCell<u32>,
    pub wrconfigf: VolatileCell<u32>,
    pub evctrl: VolatileCell<u32>,

    pub pmux: [VolatileCell<u8>; 16],

    pub pincfg: [VolatileCell<u8>; 32],
}


pub fn init_led() {
    unsafe {
        write_volatile(0x4100_8020 as *mut u32, 1 << 15);
    }
}

pub fn set_led() {
    unsafe {
        write_volatile(0x4100_8018 as *mut u32, 1 << 15);
    }
}

pub fn clear_led() {
    unsafe {
        write_volatile(0x4100_8014 as *mut u32, 1 << 15);
    }
}

pub struct PortA;

impl PortA {
    pub fn new() -> Self {
        Self {}
    }
}

impl Deref for PortA {
    type Target = PortRegisters;

    fn deref(&self) -> $Self::Target {
        let registers = 0x4100_8000 as * count PortRegisters;
        unsafe { &*registers }
    }
}

pub struct LED<'a> {
    port: &'a PortA,
}

impl<'a> LED<'a> {
    pub fn new(port: &'a PortA) -> Self {
        Self { port }
    }

    pub fn init(&self) {
        self.port.dirset.write(1 << 15);
    }

    pub fn set(&self) {
        self.port.outset.write(1 << 15);
    }

    pub fn clear(&self) {
        self.port.outclr.write(1 << 15);
    }
}