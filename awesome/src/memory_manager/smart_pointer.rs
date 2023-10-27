/// ===================== Box ================== 
/// 最简单直接的智能指针是 box，其类型是 Box<T>。box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。如果你想回顾一下栈与堆的区别请参考第四章。
// 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
// 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

// fn mybox() {
//     let b = Box::new(5);
//     println("b = {}",b);
// }


// Box 允许创建递归类型
// 递归类型（recursive type）的值可以拥有另一个同类型的值作为其的一部分。
// 这会产生一个问题因为 Rust 需要在编译时知道类型占用多少空间。递归类型的值嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。
// 因为 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// fn test_List() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// ============== 自定义智能指针 =============
// 为了体会默认情况下智能指针与引用的不同，让我们创建一个类似于标准库提供的 Box<T> 类型的智能指针。接着学习如何增加使用解引用运算符的功能。
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// MyBox<T> 类型不能解引用，因为我们尚未在该类型实现这个功能。为了启用 * 运算符的解引用功能，需要实现 Deref trait
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//输入 *y 时，Rust 事实上在底层运行了如下代码：
// *(y.deref())


// ====================================使用 Drop Trait 运行清理代码=============================
// 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。
// Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。为了能够看出 Rust 何时调用 drop，让我们暂时使用 println! 语句实现 drop
struct CoustomSmartPointer {
    data : String,
}

impl Drop for CoustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


// << 通过 std::mem::drop 提早丢弃值 >> 
/// 以下代码会报错
//  let c = CustomSmartPointer {
// data: String::from("some data"),
// };
// println!("CustomSmartPointer created.");
// c.drop(); // error[E0040]: explicit use of destructor method
// println!("CustomSmartPointer dropped before the end of main.");

// Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。

// 因为不能禁用当值离开作用域时自动插入的 drop，并且不能显式调用 drop，如果我们需要强制提早清理值，可以使用 std::mem::drop 函数。

// std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望强制丢弃的值作为参数。std::mem::drop 位于 prelude，所以我们可以修改示例 15-15 中的 main 来调用 drop 函数。如示例

pub fn mem_drop_run() {
    let c = CoustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

pub mod reference_count {
    // Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。
    // 如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。
    // 注意 Rc<T> 只能用于单线程场景；第十六章并发会涉及到如何在多线程程序中进行引用计数。
    
    use std::rc::Rc;
    pub fn run_rc_test() {
        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }    
        // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
        // let b = List::Cons(3, Box::new(a));
        // let c = List::Cons(4, Box::new(a)); It will introduce a error 
        // we relace the above code with the below code
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        // Rc::clone 只会增加引用计数，这并不会花费多少时间
        {
            let c = List::Cons(3, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        let b = List::Cons(4, Rc::clone(&a));
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

pub mod ref_cell {
    // 类似于 Rc<T>，RefCell<T> 只能用于单线程场景。如果尝试在多线程上下文中使用RefCell<T>，会得到一个编译错误。第十六章会介绍如何在多线程程序中使用 RefCell<T> 的功能。

    // 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
    
    // Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
    // Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
    // 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
    
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
    
            let percentage_of_max = self.value as f64 / self.max as f64;
    
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
    
    #[cfg(test)]
    // mod tests {
    //     use super::*;
    //     use std::cell::RefCell;
    
    //     struct MockMessenger {
    //         sent_messages: Vec<String>,
    //     }
    
    //     impl MockMessenger {
    //         fn new() -> MockMessenger {
    //             MockMessenger {
    //                 sent_messages: vec![],
    //             }
    //         }
    //     }
    
    //     impl Messenger for MockMessenger {
    //         fn send(&self, message: &str) {
    //             self.sent_messages.push(String::from(message));
    //         }
    //     }
    
    //     #[test]
    //     fn it_sends_an_over_75_percent_warning_message() {
    //         let mock_messenger = MockMessenger::new();
    //         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
    //         limit_tracker.set_value(80);
    
    //         assert_eq!(mock_messenger.sent_messages.len(), 1);
    //     }
    // }
    
    
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;
    
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }
    
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }
    
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }
    
        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            // --snip--
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    
            limit_tracker.set_value(80);
    
            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }
    }
    
    
}
pub mod tests {

    use crate::memory_manager::{smart_pointer::*, self};
    use crate::memory_manager::smart_pointer::reference_count::run_rc_test;
    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // ERROR: type `smart_pointer::MyBox<{integer}>` cannot be dereferenced

    }

    #[test]
    fn test_frop() {
        let c = CoustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CoustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    #[test]
    fn test_run_rc() {
        run_rc_test();
    }
}

// 在 Rust 中，凡是需要做资源回收的数据结构，且实现了 Deref/DerefMut/Drop，都是智能指针。
pub mod StringTest {

    // 那么又有一个问题了，智能指针和结构体有什么区别呢？因为我们知道，String 是用结构体定义的：
    pub struct String { vec: Vec<u8>,}
    // impl ops::Deref for String {
    //     type Target = str;
    
    //     fn deref(&self) -> &str {
    //         unsafe { str::from_utf8_unchecked(&self.vec) }
    //     }
    // }
    
    // impl ops::DerefMut for String {
    //     fn deref_mut(&mut self) -> &mut str {
    //         unsafe { str::from_utf8_unchecked_mut(&mut *self.vec) }
    //     }
    // }


    // unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
    //     fn drop(&mut self) {
    //         unsafe {
    //             // use drop for [T]
    //             // use a raw slice to refer to the elements of the vector as weakest necessary type;
    //             // could avoid questions of validity in certain cases
    //             ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
    //         }
    //         // RawVec handles deallocation
    //     }
    // }
    
}