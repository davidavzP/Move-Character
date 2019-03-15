#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bare_metal_1::println;
use bare_metal_1::interrupts::PICS;
use bare_metal_1::move_char;
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
//    let vga_buffer = 0xb8000 as *mut u8;
//    let O: &[u8] = b"0";
//
//
//        unsafe {
//            *vga_buffer.offset( 300 as isize * 2) = *O.get(0).unwrap();
//            *vga_buffer.offset(300 as isize * 2 + 1) = 0xb;
//        }

    //puts a char to the middle of the screen


    move_char::MOVER.lock().place_char(b'0');
    println!("Hello world!");
    println!("USE THE W, S, A, D, keys to move the zero around the screen :)");

    bare_metal_1::interrupts::init_idt();
    unsafe {
        PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

