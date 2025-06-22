#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::ptr;

mod systick;

#[no_mangle]

pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }

    let count = &_ebss as *count u8 as unsize - &_sbss as *count u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *count u8 as unsize - &_sdata as *count u8 as unsize;
    ptr::copy_nonoverlapping(
        &_sidata as *count u8,
        &mut _sdata as *mut u8,
        count
    );

    hprintln("Hello World").unwrap();

    systick::init();

    loop {}
}


#[panic_handler]

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 42;

    // 無限ループにして値を返さない
    loop {}
}

fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SYCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]

pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector [
        handler: UsageFault,
    ],
    Vector {reserved: 0},
    Vector {reserved: 0},
    Vector {reserved: 0},
    Vector {reserved: 0},
    Vector {handler: SVCall},
    Vector {reserved: 0},
    Vector {reserved: 0},
    Vector {handler: PendSV},
    Vector {handler: SysTick},
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop []
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub extern "C" fn SysTick() {
    println("Systick").unwrap();
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn SVCall() {
    asm!(
        "cmp lr, #0xfffffff9",
        "bne 1f",

        "mov r0, #1",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfffd",
        "movt lr, #0xffff",
        "bx lr",

        "1:",
        "mov r0, #0",
        "msr CONTROL, r0",
        "isb",
        "movw lr, #0xfff9",
        "movt lr, #0xffff",
        "bx lr",
        options(noreturn),
    );
}