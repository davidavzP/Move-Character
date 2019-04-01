extern crate lazy_static;
extern crate spin;
extern crate volatile;



use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::vga_buffer::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//sets up the buffer size
const MAZE_HEIGHT: usize = 25;
const MAZE_WIDTH: usize = 80;


//this creates the buffer in a 2-d array

//struct MAZE {
//    maze_char: [usize; MAZE_HEIGHT*MAZE_WIDTH],
//    num_visited_cells: usize,
//    stack: List<Pair>
//
//}
//
//
//struct Pair{
//    x: usize,
//    y: usize
//
//}
////https://www.reddit.com/r/rust/comments/2j2li2/implementing_simple_data_strutures_in_rust/
////User: Gankro
//struct Node<T> {
//    data: T,
//    next: Option<Box<Node<T>>>
//}
//
//trait A { type B; }
//
//struct List<T: A> {
//    head: Option<Box<Node<T>>>
//}
//
//impl<T: A> List<T> {
//    fn new() -> List<T> {
//        List { head: None }
//    }
//
//    fn push(&mut self, elem: T) {
//        self.head = Some(box Node {
//            data: elem,
//            next: self.head.take(),
//        });
//    }
//
//    fn pop(&mut self) -> Option<T> {
//        match self.head.take() {
//            None => None,
//            Some(mut head) => {
//                self.head = head.next.take();
//                Some(head.data)
//            }
//        }
//    }
//}
