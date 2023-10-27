use std::mem;
use std::mem::ManuallyDrop;
  
fn read_usize(x: &[u8]) -> usize {
    // The length of usize is 64 = [u8;8]
    assert!(x.len() >= mem::size_of::<usize>());
   
    let ptr = x.as_ptr() as *const usize;
    //此处必须用ptr::read_unaligned，因为不确定字节是否对齐
    unsafe { ptr.read_unaligned() }
}

fn manually_drop(){
    let mut x = ManuallyDrop::new(String::from("Hello World!"));
    x.truncate(5); // 此时会调用deref
    assert_eq!(*x, "Hello");
    // 但对x的drop不会再发生
}


fn test_ptr() {
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;

    let mut speed: Box<i32> = Box::new(1);
    let speed_ptr: *mut i32 = &mut *speed;

    let speed1: Box<i32> = Box::new(1);
    let speed_ptr1: *const i32 = &*speed1;
}

#[test]
fn test_read_usize(){
    let arr1:[u8;8] = [1,2,3,4,5,6,7,8];
    let read_val = read_usize(&arr1); 
    println!("read_val: {}",read_val);
}