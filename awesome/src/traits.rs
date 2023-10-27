/// 函数，是把重复代码中的参数抽取出来，使其更加通用，调用函数的时候，根据参数的不同，我们得到不同的结果；
/// 而泛型，是把重复数据结构中的参数抽取出来，在使用泛型类型时，根据不同的参数，我们会得到不同的具体类型
/// 
/// 再来看一个复杂一点的泛型结构 Vec 的例子，验证一下这个想法：
// pub struct Vec<T, A: Allocator = Global> {
//     buf: RawVec<T, A>,
//     len: usize,
// }

// pub struct RawVec<T, A: Allocator = Global> {
//     ptr: Unique<T>,
//     cap: usize,
//     alloc: A,
// }

/// Vec有两个参数，一个是 T，是列表里的每个数据的类型，另一个是 A，它有进一步的限制 A: Allocator ，也就是说 A 需要满足 Allocator trait。
/// A 这个参数有默认值 Global，它是 Rust 默认的全局分配器，这也是为什么 Vec 虽然有两个参数，使用时都只需要用 T。
/// 
/// 在讲生命周期标注的时候，我们讲过，数据类型内部如果有借用的数据，需要显式地标注生命周期。
/// 其实在 Rust 里，生命周期标注也是泛型的一部分，一个生命周期 'a 代表任意的生命周期，和 T 代表任意类型是一样的。
/// 
/// 
/// 
/// pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
/// {
///     // 借用的数据
///     Borrowed(&'a B),
///     // 拥有的数据
///     Owned(<B as ToOwned>::Owned),
/// }
/// Cow（Clone-on-Write）是 Rust 中一个很有意思且很重要的数据结构。
/// 它就像 Option 一样，在返回数据的时候，提供了一种可能：要么返回一个借用的数据（只读），要么返回一个拥有所有权的数据（可写）
/// 对于拥有所有权的数据 B ，第一个是生命周期约束。这里 B 的生命周期是 'a，所以 B 需要满足 'a，这里和泛型约束一样，也是用 B: 'a 来表示。
/// 当 Cow 内部的类型 B 生命周期为 'a 时，Cow 自己的生命周期也是 'a。B 还有两个约束：?Sized 和 “where B: ToOwned”
/// 
/// ?Sized 是一种特殊的约束写法，? 代表可以放松问号之后的约束。由于 Rust 默认的泛型参数都需要是 Sized，也就是固定大小的类型，所以这里 ?Sized 代表用可变大小的类型
/// 
/// ToOwned 是一个 trait，它可以把借用的数据克隆出一个拥有所有权的数据。
/// 
/// 所以这里对 B 的三个约束分别是：生命周期 'a长度可变 ?Sized符合 ToOwned trait
/// 
/// 
/// 
/// <<我们也可以在不同的实现下逐步添加约束>>
use std::fs::File;
use std::io::{BufReader, Read, Result};

// 定义一个带参数类型R的reader, 此处我们不限制R
struct MyReader<R> {
    reader: R,
    buf: String,
}

// 实现new 函数的时候 ，我们不需要限制R
impl<R> MyReader<R> {
    pub fn new(reader: R) -> MyReader<R> {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

// 定义process的时候 我们需要用到R方法 
impl<R> MyReader<R> 
where R:Read 
{
    pub fn process(&mut self)->Result<usize>{
        self.reader.read_to_string(&mut self.buf)
    }
}

pub mod test {

    use crate::traits::*;
    use std::io::{BufReader, Read, Result};

    #[test]
    fn test_read(){
        let  f = File::open("/Users/chenyanlong/Desktop/enjoy/rust-learning/awesome/Cargo.toml").unwrap();
        let mut reader = MyReader::new(BufReader::new(f));

        let size = reader.process().unwrap();
        println!("total size: {:?}", size);
    }
}