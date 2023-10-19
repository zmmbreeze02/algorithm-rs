pub mod binary_search;
pub mod binary_search_left;
pub mod binary_search_right;
pub mod ringbuffer;
pub mod atomic;
pub mod knuth_shuffle;

use std::{alloc::{alloc, Layout}, ptr, cell::RefCell, thread};
use ringbuffer::RingBuffer;

struct TreeNode<T>{  
	value: Option<T>,  
	children: Vec<RefCell<TreeNode<T>>>,  
	parent: Option<Box<RefCell<TreeNode<T>>>>,
}

unsafe fn get_val() -> *const i32 {
    let mut my_num: Box<i32> = Box::new(10);
    &*my_num
}

fn main() {
    // let mut tree_root = TreeNode {
    //     value: Option::Some(1),
    //     children: Vec::new(),
    //     parent: Option::None,
    // };
    // // tree_root.children.push(TreeNode { value: Some(2), children: Vec::new(), parent: Some(&tree_root) });

    // println!("Hello world");
    // let layout = Layout::array::<u32>(4).unwrap();
    // unsafe {
    //     let buf = alloc(layout) as *mut u32;
    //     ptr::write(buf, 1);
    //     ptr::write(buf.wrapping_add(1), 2);
    //     ptr::write(buf.wrapping_add(2), 3);
    //     ptr::write(buf.wrapping_add(3), 4);
    //     ptr::write(buf.add(4), 5);
    //     println!("{}", *buf);
    //     println!("{}", *(buf.add(1)));
    // }

    // let data1 = String::from("Hello");
    // // let a: &String = &data1;
    // // let mut data2 = String::from("World");
    // // let b: &mut String = &mut data2;
    // let a: &str = data1.as_ref();


    // let mut my_num: Box<i32> = Box::new(10);
    // let ref1 = my_num.as_ref();
    // // let ref2 = my_num.as_mut();
    // // *ref2 = 3;
    // // let ref3 = &mut *my_num;
    // let my_num_ptr: *const i32 = &*my_num;
    // // let my_num_ptr = Box::into_raw(my_num);
    // unsafe {
    //     println!("> {ref1} {} {}", *my_num_ptr, my_num);
    //     println!("> {}", *get_val());
    // }

    // let my = Box::new(1);
    // let aa = &my;
    // println!("{aa}");

    // atomic::test1();

    // let RB: RingBuffer<u32> = RingBuffer::with_capacity(32);
    // let a = thread::spawn(move || {

    // });
    // let b = thread::spawn(move || {

    // });
    
}
