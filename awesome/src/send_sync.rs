use std::{
    mem::{align_of, size_of},
    ptr,
    cmp::max,
};

// struct Carton<T>(ptr::NonNull<T>);

// impl  <T> Carton<T>{
//     pub fn new(value: T) -> Self {
//         // Allocate enough memory on the heap to store one T. 
//         assert_ne!(size_of::<T>(),0 ,"zero sized types are not allowed");

//         let mut memptr: *mut T = ptr::null_mut();

//         unsafe {
//             let ret = libc::posix_memalign(

//             );
//         }
//     }
// }