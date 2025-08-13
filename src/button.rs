pub struct Button<'a, const N: usize> {
    pin: &'a Pin<PortC, N>,
}

impl<'a, const N: usize> Button<'a, N> {
    pub fn init(&self) {
        self.pin.clear_dir();
        self.pin.enable_floating_input();
    }

    pub fn is_pushed(&self) -> bool {
        !self.pin.get_in()
    }
}