#![no_main]
#![no_std]

extern crate alloc;

use core::fmt::Alignment;
use core::panic::PanicInfo;
use core::ptr;
use core::mem::MaybeUninit;
use process::{AligedStack, Process};
use led::{PortA, LED};
use alloc::alloc::{GlobalAlloc, Layout};

struct DummyAllocator;

mod systick;
mod led;


#[link_section = ".app_stack"]
static mut APP_STACK: AlignedStack = AligedStack(MaybeUninit::uninit());

let mut process = Process::new(&mut APP_STACK, app_main);
process.exec();

hprintln("Kernel").unwrap();


#[repr(align(8))]
struct AlignedStack(MaybeUninit<[u8; 1024]>);

extern "C" {
    fn asm_execute_process(sp: usize);
}

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


    context_frame.r0 = 0;
    context_frame.r1 = 0;
    context_frame.r2 = 0;
    context_frame.r3 = 0;
    context_frame.r12 = 0;
    context_frame.lr = 0;
    context_frame.return_addr = app_main as u32;
    context_frame.xpsr = 0x0100_0000;

    adm_execute_process(ptr);

    systick::init();

    loop {}

    led::init_led();
    hprintln!("Set LED").unwrap();
    led::set_led();
    hprintln!("Clear LED").unwrap();
    led::clear_led();


    let porta = PortA::new();
    let led = LED::new(&porta);
    led.init();
    hprintln!("Set LED").unwrap();
    led.set();
    hprintln!("Clear LED").unwrap();
    led.clear();

    
    sched.exec();
}

extern "C" fn app_main() -> ! {
    hprintln!("App").unwrap();
    unsafe { asm!("svc 0"); }
    loop {}
}

extern "C" fn app_main2() -> ! {
    loop {
        hprintln!("App2").unwrap();
        unsafe { asm!("svc 0"); }
    }
}

extern "C" fn app_main3() -> ! {
    loop {
        hprintln!("App: {}", i).unwrap();
        unsafe { asm("svc 0"); }
    }
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

    #[link_section = ".app_stack"]
    static mut APP_STACK: AligedStack = AlignedStack(MaybeUninit::uninit());
}

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!();
    }

    unsafe fn dealloc(&self, _ptr: &mut u8, _layout: Layout) {
        unimplemented!();
    }
}

[#global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;

#![feature(alloc_error_handler)]

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!();
}