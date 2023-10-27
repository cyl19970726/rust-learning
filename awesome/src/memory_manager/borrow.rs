use std::ptr; 
use std::mem;
// Vec的数据结构很特殊
// |ptr|len|cap|  = Vec_Struct 
// &vec 会得到 ptr，指向堆存储数据的地址 
// &&vec 才会得到Vec_Struct在 memory的地址
fn data_ptr_1() {
    println!(">> Data Ptr 1 <<");
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    // 内存地址
    // data1 的值地址等于data的ptr
    println!(
        "&data,data1: {:p}({:p}),  &&data {:p}, data1 ptr: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );

}

fn data_ptr_2() {
    println!(">> Data Ptr 2 <<");
    let mut data = vec![1, 2, 3, 4];
    let data1 = &mut data;
    // 值的地址是什么？引用的地址又是什么？
    // 内存地址
    // data1 的值地址等于data的ptr
    println!(
        "data1 {:p}, &data1: {:p}",
        data1, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );

}

// pass the heap addr 
fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("sum argument: &Vec<u32>: {:p}, &&Vec<u32>: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

fn data_ptr_3(){
    let mut data: &Vec<i32> = &vec![1, 2, 3, 4];
    let data1 = data;

    assert_eq!(data.as_ptr(),&data[0]);

    println!("{:p} , {:p}",data1,data);
    println!("{:p} , {:p}",&data[0],&data[1]);
}

fn data_ptr_4() {
    let v = vec![1, 2, 3];

    // FIXME Update this when vec_into_raw_parts is stabilized
    // Prevent running `v`'s destructor so we are in complete control
    // of the allocation.
    let mut v = mem::ManuallyDrop::new(v);
    
    // Pull out the various important pieces of information about `v`
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    
    unsafe {
        // Overwrite memory with 4, 5, 6
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
        
}


fn data_ptr_5(){
    use std::alloc::{alloc, Layout};

    let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

    let vec = unsafe {
        let mem = alloc(layout).cast::<u32>();
        if mem.is_null() {
            return;
        }

        mem.write(1_000_000);

        Vec::from_raw_parts(mem, 1, 16)
    };

    assert_eq!(vec, &[1_000_000]);
    println!("{:?}",vec[0]);
    assert_eq!(vec.capacity(), 16);
}
/// The `Vec` type.
/// pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
///     buf: RawVec<T, A>,
///     len: usize,
/// }
/// 
/// pub(crate) struct RawVec<T, A: Allocator = Global> {
///     ptr: Unique<T>,
///     cap: usize,
///     alloc: A,
/// }
/// 
///  >>> invoke chain <<<
/// Vec 
///  pub fn with_capacity(capacity: usize) -> Self {
///     Self::with_capacity_in(capacity, Global)
///  }
///  pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
///     Vec { buf: RawVec::with_capacity_in(capacity, alloc), len: 0 }
/// }
/// 
/// Raw_Vec 
/// Like `with_capacity`, but parameterized over the choice of
/// allocator for the returned `RawVec`.
/// #[cfg(not(no_global_oom_handling))]
/// #[inline]
/// pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
///     Self::allocate_in(capacity, AllocInit::Uninitialized, alloc)
/// }
/// 
/// The allocate_in function use the`layouter` to allocate the memory, 
/// and finally mark the allcated memory as a ptr represented as Unique::new_unchecked(ptr.cast().as_ptr()) 
/* 
#[cfg(not(no_global_oom_handling))]
fn allocate_in(capacity: usize, init: AllocInit, alloc: A) -> Self {
    // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.
    if T::IS_ZST || capacity == 0 {
        Self::new_in(alloc)
    } else {
        // We avoid `unwrap_or_else` here because it bloats the amount of
        // LLVM IR generated.
        let layout = match Layout::array::<T>(capacity) {
            Ok(layout) => layout,
            Err(_) => capacity_overflow(),
        };
        match alloc_guard(layout.size()) {
            Ok(_) => {}
            Err(_) => capacity_overflow(),
        }
        let result = match init {
            AllocInit::Uninitialized => alloc.allocate(layout),
            AllocInit::Zeroed => alloc.allocate_zeroed(layout),
        };
        let ptr = match result {
            Ok(ptr) => ptr,
            Err(_) => handle_alloc_error(layout),
        };

        // Allocators currently return a `NonNull<[u8]>` whose length
        // matches the size requested. If that ever changes, the capacity
        // here should change to `ptr.len() / mem::size_of::<T>()`.
        Self {
            ptr: unsafe { Unique::new_unchecked(ptr.cast().as_ptr()) },
            cap: capacity,
            alloc,
        }
    }
}
*/

pub mod tests {
    use crate::memory_manager::borrow::*;

    #[test]
    fn test_borrow_1() {
        data_ptr_1();
    }

    #[test]
    fn test_borrow_2() {
        data_ptr_2();
    }

    #[test]
    fn test_borrow_3(){
        // data_ptr_3();
        // data_ptr_4();
        data_ptr_5();
    }
}