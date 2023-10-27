
pub mod closure;
pub mod traits;
pub mod abstarct_trait;
pub mod send_sync;
pub mod life_time;
pub mod memory_manager;
pub mod container;
// pub mod reference_count;

use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

// 第一种方法

// impl<W: Write> MyWriter<W> {
//     pub fn new(addr: &str) -> MyWriter<BufWriter<TcpStream>>  {
//         let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//         MyWriter {
//             writer: BufWriter::new(stream),
//         }
//     }

//     pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//         self.writer.write_all(buf.as_bytes())
//     }

// }



// 第二种方法
// 可以对不同具体类型实现多个new方法
// impl MyWriter<BufWriter<TcpStream>> {
//     pub fn new(addr: &str) -> Self {
//         let stream = TcpStream::connect(addr).unwrap();
//         Self {
//             writer: BufWriter::new(stream),
//         }
//     }

//     pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
//         self.writer.write_all(buf.as_bytes())
//     }
// }


// 第三种方法

impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> Self {
        Self{
            writer,
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}


fn main() {
    // let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
    // writer.write("hello world!");

    // let mut writer1 = MyWriter::new("127.0.0.1:8080");
    // writer1.write("hello world!");

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut writer = MyWriter::new(BufWriter::new(stream));
    writer.write("hello world!");
}
