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

// 介绍过只读引用实现了 Copy trait，也就意味着引用的赋值、传参都会产生新的浅拷贝。
// data 有很多只读引用指向它，但堆上的数据依旧只有 data 一个所有者，所以值的任意多个引用并不会影响所有权的唯一性。


// 引出问题： 一旦 data 离开了作用域被释放，如果还有引用指向 data，岂不是造成我们想极力避免的使用已释放内存（use after free）这样的内存安全问题？怎么办呢？


/*
>> 借用的生命周期及其约束 << 
我们对值的引用也要有约束，这个约束是：借用不能超过（outlive）值的生存期。

fn test_ref_lifetime() {
    let r = local_ref();
    println!("r: {:p}", r);
}

fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    &a
}


****** *****
*/

fn test_ref_lifetime_2(){
    // 可变数组存放在堆上，栈上只有一个胖指针指向它，所以这是一个典型的把栈上变量的引用存在堆上
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}

// fn test_ref_lifetime_3(){
//     fn main() {
//         let mut data: Vec<&u32> = Vec::new();
//         push_local_ref(&mut data);
//         println!("data: {:?}", data);
//     }
    
//     fn push_local_ref(data: &mut Vec<&u32>) {
//         let v = 42;
//         data.push(&v);
//     }
    
// }

//但如果抓住了一个核心要素“在一个作用域下，同一时刻，一个值只能有一个所有者”，你会发现，其实很简单。
//堆变量的生命周期不具备任意长短的灵活性，因为堆上内存的生死存亡，跟栈上的所有者牢牢绑定。而栈上内存的生命周期，又跟栈的生命周期相关，所以我们核心只需要关心调用栈的生命周期。

/* 
===== 多个可变引用共存 会报错 =======
fn can_modify_mut() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {
        data.push(*item + 1); // 在同一个作用域下有多个可变引用，是不安全的。
    }
}

*/

/*
同时有一个可变引用和若干个只读引用，
*/
// fn test_borrow_read_write(){
//     let mut data = vec![1, 2, 3];

//     let data1 = vec![&data[0]];
//     println!("data[0]: {:p}", &data[0]);

//     for i in 0..100{
//         data.push(i); //如果你仔细推敲，就会发现这里有内存不安全的潜在操作：如果继续添加元素，堆上的数据预留的空间不够了，就会重新分配一片足够大的内存，把之前的值拷过来，然后释放旧的内存。这样就会让 data1 中保存的 &data[0] 引用失效，导致内存安全问题。
//     }

//     println!("data[0]: {:p}", &data[0]);
//     println!("boxed: {:p}", &data1);
// }

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

    #[test]
    fn test_ref(){
        test_ref_lifetime_2();
    }
}