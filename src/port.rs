pub struct Pin<P: PortId, const N: usize> {
    _port_id: PhantomData<p>,
}

trait PortId {
    const ADDR: usize;
}

pub struct PortA {}
pub struct PortC {}

pub struct Port<P: PortId> {
    pub pin0: Pin<P, 0>,
    pub pin1: Pin<P, 1>,
    pub pin2: Pin<P, 2>,
    pub pin3: Pin<P, 3>,
    pub pin4: Pin<P, 4>,
    pub pin5: Pin<P, 5>,
    pub pin6: Pin<P, 6>,
    pub pin7: Pin<P, 7>,
    pub pin8: Pin<P, 8>,
    pub pin9: Pin<P, 9>,
    pub pin10: Pin<P, 10>,
    pub pin11: Pin<P, 11>,
    pub pin12: Pin<P, 12>,
    pub pin13: Pin<P, 13>,
    pub pin14: Pin<P, 14>,
    pub pin15: Pin<P, 15>,
    pub pin16: Pin<P, 16>,
    pub pin17: Pin<P, 17>,
    pub pin18: Pin<P, 18>,
    pub pin19: Pin<P, 19>,
    pub pin20: Pin<P, 20>,
    pub pin21: Pin<P, 21>,
    pub pin22: Pin<P, 22>,
    pub pin23: Pin<P, 23>,
    pub pin24: Pin<P, 24>,
    pub pin25: Pin<P, 25>,
    pub pin26: Pin<P, 26>,
    pub pin27: Pin<P, 27>,
    pub pin28: Pin<P, 28>,
    pub pin29: Pin<P, 29>,
    pub pin30: Pin<P, 30>,
    pub pin31: Pin<P, 31>,
}

impl<P: PortId> Port<P> {
    pub fn new() -> Self {
        Self {
            pin0: Pin::new(),
            pin1: Pin::new(),
            pin2: Pin::new(),
            pin3: Pin::new(),
            pin4: Pin::new(),
            pin5: Pin::new(),
            pin6: Pin::new(),
            pin7: Pin::new(),
            pin8: Pin::new(),
            pin9: Pin::new(),
            pin10: Pin::new(),
            pin11: Pin::new(),
            pin12: Pin::new(),
            pin13: Pin::new(),
            pin14: Pin::new(),
            pin15: Pin::new(),
            pin16: Pin::new(),
            pin17: Pin::new(),
            pin18: Pin::new(),
            pin19: Pin::new(),
            pin20: Pin::new(),
            pin21: Pin::new(),
            pin22: Pin::new(),
            pin23: Pin::new(),
            pin24: Pin::new(),
            pin25: Pin::new(),
            pin26: Pin::new(),
            pin27: Pin::new(),
            pin28: Pin::new(),
            pin29: Pin::new(),
            pin30: Pin::new(),
            pin31: Pin::new(),
        }
    }
}

impl PortId for PortA {
    const ADDR: usize = 0x4100_8000;
}

impl PortId for PortC {
    const ADDR: usize = 0x4100_8100;
}

impl<P: PortId, const N: usize> Pin<P, N> {
    fn new() -> Self {
        Self { _port_id: PhantomData }
    }

    fn registers<'a>(&'a self) &'a PortRegisters {
        let registers = P::ADDR as *const PortRegisters;
        unsafe { &*registers }
    }

    fn clear_dir(&self) {
        self.registers().dirclr.write(1 << N);
    }

    fn set_dir(&self) {
        self.registers().dirset.write(1 << N);
    }

    fn set_out(&self) {
        self.registers().outset.write(1 << N);
    }

    fn clear_out(&self) {
        self.registers().outclr.write(1 << N);
    }
 }
