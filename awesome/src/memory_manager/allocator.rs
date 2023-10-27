// use std::alloc::{GlobalAlloc, Layout, System};

// struct MyAllocator;

// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let data = System.alloc(layout);
//         // 首先看内存的分配。这里 MyAllocator 就用 System allocator，
//         // 然后加 eprintln!()，和我们常用的 println!() 不同的是，eprintln!() 将数据打印到 stderr
//         // 注意这里不能使用 println!() 。因为 stdout 会打印到一个由 Mutex 互斥锁保护的共享全局 buffer 中，这个过程中会涉及内存的分配，
//         // 分配的内存又会触发 println!()，最终造成程序崩溃。而 eprintln! 直接打印到 stderr，不会 buffer。
//         eprintln!("ALLOC: {:p}, size {}", data, layout.size());
//         data
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         System.dealloc(ptr, layout);
//         eprintln!("FREE: {:p}, size {}", ptr, layout.size());
//     }
// }

// #[global_allocator]
// static GLOBAL: MyAllocator = MyAllocator;

// #[allow(dead_code)]
// struct Matrix {
//     // 使用不规则的数字如 505 可以让 dbg! 的打印很容易分辨出来
//     data: [u8; 505],
// }

// impl Default for Matrix {
//     fn default() -> Self {
//         Self { data: [0; 505] }
//     }
// }

// #[test]
// fn main() {
//     // 在这句执行之前已经有好多内存分配
//     let data = Box::new(Matrix::default());

//     // 输出中有一个 1024 大小的内存分配，是 println! 导致的
//     println!(
//         "!!! allocated memory: {:p}, len: {}",
//         &*data,
//         std::mem::size_of::<Matrix>()
//     );

//     // data 在这里 drop，可以在打印中看到 FREE
//     // 之后还有很多其它内存被释放
// }