#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bare_metal_1::println;
use bare_metal_1::interrupts::PICS;
use bare_metal_1::move_char;
static HELLO: &[u8] = b"Hello World!";



#[no_mangle]
pub extern "C" fn _start() -> ! {

    //puts a char to the middle of the screen

    for _i in 0..5{
    println!("");
    println!("None of them noticed a large, tawny owl flutter past the window. ");
    for _i in 0..4{
        println!("");
    }
    println!("None of them noticed a large, tawny owl flutter past the window. ");
        }
    println!("Hello world!");
    println!("USE THE W, S, A, D, keys to move the zero around the screen :)");

    bare_metal_1::interrupts::init_idt();
    unsafe {
        PICS.lock().initialize()
    };
    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    move_char::MOVER.lock().place_moving_char();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

