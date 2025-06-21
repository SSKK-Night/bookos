use cortex_m_semihosting::hprintln;

pub fn init() {
    hprintln("Systick init").unwrap();
}