pub fn syscall_yield() {
    unsafe {
        asm("svc 0", in("r0") 0);
    }
}

pub fn syscall_set_led(value: bool) {
    unsafe {
        asm!("svc 0", in("r0") 1, in("r1") value as u32);
    }
}