
// >> 1. the max need to specific life cyclt for two arguments <<
pub fn run_max_str() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}


fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}


// >> 1. compiler help us to sign simple life cycle <<
fn extract_first_word(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

fn extract_first_word_with_lc<'a>(s: &'a str) -> &'a str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

pub fn run_ef() {
    let s1 = "Make the word beatter by coding!";
    println!("first world of s1:{}",extract_first_word(&s1));

    println!("second world of s1:{}",extract_first_word(&s1));
}



// >> 3. Is the lifetime the lifetime of a reference's reference or the lifetime of a reference << 
// you can view the image about the relation between the &mut and &str : ../image/mut_str.webp


// pub fn strtok(s: &mut &str, delimiter: char) -> &str {
//     if let Some(i) = s.find(delimiter) {
//         let prefix = &s[..i];
//         // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
//         // 直接使用 len 返回的是字节长度，会有问题
//         let suffix = &s[(i + delimiter.len_utf8())..];
//         *s = suffix;
//         prefix
//     } else { // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
//         let prefix = *s;
//         *s = "";
//         prefix
//     }
// }

// 会把字符串安装分隔符(delimiter)切出一个token 并且返回，然后将传入的字符串指向后续的token
// 这里需要思考返回值和谁的生命周期有关，一个是指向字符串引用(&str)的可变引用&mut; 还是字符串引用&str;
// 显然应该是跟字符串引用 &str 保持一致
pub fn strotk<'b , 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else { // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = "";
        prefix
    }
}

pub fn run_strotk() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strotk(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}



// 4. the life cycle of the struct should shorter or euqal with their field.
struct Employee<'a, 'b> {
    name: &'a str,
    title: &'b str,
    age: u8,
  }