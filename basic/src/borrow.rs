pub fn bo1() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );

    let ptr = data.as_ptr();
    println!("vec ptr: {:p} ; addr of ptr: {:p}",ptr, &ptr);
    // println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    print_vec("vec heap:",data)
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}


// fn bro2() {
//     let mut data = vec![1, 2, 3];
//     let mut data1 = vec![&data[0]];
//     println!("data[0]: {:p}", &data[0]);

//     for i in 0..100 {
//         data.push(i); // error  mutable borrow occurs here
//     }

//     println!("data[0]: {:p}", &data[0]);
//     println!("boxed: {:p}", &data1);
// }

use std::mem;

pub fn bro3() {
    // capacity 是 1, len 是 0
    let mut v = vec![1];
    // capacity 是 8, len 是 0
    let v1: Vec<i32> = Vec::with_capacity(8);

    print_vec("v1", v1);

    // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
    println!("heap start: {:p}", &v[0] as *const i32);

    extend_vec(&mut v);

    // heap 地址改变了！这就是为什么可变引用和不可变引用不能共存的原因
    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);
}

fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe { mem::transmute(data) };
    // 打印 Vec<T> 的堆地址，capacity，len
    println!("{}: 0x{:x}, capacity:{}, len:{}", name, p[0], p[1], p[2]);
}


struct People {
    age: u64,
    name: str,
}

impl People {
    fn new(age: u64, name: str) -> Self{
        Self {
            age: age,
            name: name,
            Assets: Vec::new(),
        }
    }
}

impl Copy for People {}
impl Clone for People {
    fn clone(&self) -> People {
        Self {
            age: self.age,
            name: self.name,
            Assets: Vec::new(),
        }
    }
}

fn bro4() {
    let p1 = &People::new(10,"aa".to_string());
    let p2 = &People::new(12,"vv".to_string());

    let mut p1_r = *p1;
    println!("{:p} {:p} {:p}",&p1_r,p1,p2)

}