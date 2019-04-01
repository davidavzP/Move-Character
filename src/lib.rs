#![feature(exclusive_range_pattern)]
#![feature(abi_x86_interrupt)]
#![feature(box_syntax)]
#![cfg_attr(not(test), no_std)]

pub mod vga_buffer;
pub mod interrupts;
pub mod move_char;
pub mod maze;



