use std::boxed::Box;
use std::alloc::{dealloc,Layout};
use std::ptr;
#[test]
fn test_1(){
    // into_raw()
    // After calling this function, the caller is responsible for the memory previously
    //  managed by the Box. In particular, the caller should properly destroy T and release 
    //  the memory, taking into account the memory layout used by Box
    //  The easiest way to do this is to convert the raw pointer back into a Box with the Box::from_raw function, 
    //  allowing the Box destructor to perform the cleanup.
    {
        let x = Box::new(5);
        let ptr = Box::into_raw(x.clone());
        let y = unsafe{ Box::from_raw(ptr)};
        assert_eq!(x,y);
    }
    

    // Manual cleanup by explicitly running the destructor and deallocating the memory:
    {
        let x = Box::new(String::from("Hello"));
        let p = Box::into_raw(x);
        unsafe{
            ptr::drop_in_place(p);
            dealloc(p as *mut u8, Layout::new::<String>());
        }
    }


    // {
    //     #![feature(allocator_api, new_uninit)]
    //     use std::alloc::System;
        
    //     let mut five = Box::<u32, _>::new_uninit_in(System);
        
    //     let five = unsafe {
    //         // Deferred initialization:
    //         five.as_mut_ptr().write(5);
        
    //         five.assume_init()
    //     };
        
    //     assert_eq!(*five, 5);
    // }

}