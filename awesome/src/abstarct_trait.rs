use std::str::FromStr;

use regex::Regex;
pub trait Parse {
    fn parse(s: &str) -> Self;
}

// 我们约束 T 必须同时实现了 FromStr 和 Default
// 这样在使用的时候我们就可以用这两个 trait 的方法了
impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // 生成一个创建缺省值的闭包，这里主要是为了简化后续代码
        // Default::default() 返回的类型根据上下文能推导出来，是 Self
        // 而我们约定了 Self，也就是 T 需要实现 Default trait
        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }
    }
}


/// 过对带有约束的泛型参数实现 trait，一份代码就实现了 u32 / f64 等类型的 Parse trait，非常精简。
/// 问题:
/// 不过，看这段代码你有没有感觉还是有些问题？当无法正确解析字符串时，我们返回了缺省值，难道不是应该返回一个错误么？


pub trait ParseUpgrade {
    type Error;
    fn parse1(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> ParseUpgrade for T
where
    T: FromStr + Default,
{
    // 定义关联类型 Error 为 String
    type Error = String;
    fn parse1(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            // 当出错时我们返回 Err(String)
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

//////////////////////////////// 支持泛型的 trait //////////////////////////// 

// pub trait Add<Rhs = Self> {
//     type Output;
//     #[must_use]
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

use std::ops::Add;
#[derive(Debug)]
struct Complex{
    real: f64,
    imagine: f64,
}

impl Complex{
    pub fn new(real: f64, imagine: f64) -> Self{ 
        Self {real, imagine}
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output{
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output{
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<f64> for &Complex {
    type Output =  Complex;
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real +rhs;
        Complex::new(real, self.imagine)
    }
}


#[derive(Debug)]
pub struct Complex2(Complex);

impl Complex2{
    pub fn new(real: f64, imagine: f64) -> Self{ 
        Complex2(Complex::new(real, imagine))
    }
}


impl Add for Complex2 {
    type Output = Complex2;

    fn add(self, rhs: Self) -> Self::Output{
        let real = self.0.real + rhs.0.real;
        let imagine = self.0.imagine + rhs.0.imagine;
        Complex2::new(real, imagine)
    }
}
mod tests {
    use crate::abstarct_trait::*;

    #[test]
    fn test_parse_should_work() {
        assert_eq!(u32::parse("123abcd"), 123);
        assert_eq!(u32::parse("123.45abcd"), 0);
        assert_eq!(f64::parse("123.45abcd"), 123.45);
        assert_eq!(f64::parse("abcd"), 0f64);
    }

    #[test]
    fn test_parse_should_work_updgrade() {   
        assert_eq!(u32::parse1("123abcd"), Ok(123));
        assert_eq!(
            u32::parse1("123.45abcd"),
            Err("failed to parse captured string".into())
        );
        assert_eq!(f64::parse1("123.45abcd"), Ok(123.45));
        assert!(f64::parse1("abcd").is_err());
    }

    #[test]
    fn test_comlplex_add() {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(2.0,3.0);
        println!("{:?}", c1 + c2); 
        // c1、c2 已经被移动，所以下面这句无法编译
         // println!("{:?}", c1 + c2);
    }
    
    #[test]
    fn test_comlplex_add_withoutMOVE() {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(2.0,3.0);
        println!("{:?}", &c1 + &c2); 
        // c1、c2 没有被移动，所以下面这句无法编译
        println!("{:?}", c1 + c2);
    }

    #[test]
    fn test_add_f64(){
        let c1 = Complex::new(1.0,1.0);

        println!("{:?}",&c1 + 6.0);
    }

    #[test]
    fn test_complex2_add(){
        let c1 =  Complex2::new(1.0,2.0);
        let c2 =  Complex2::new(1.0,2.0);
        println!("{:?}", c1 + c2); 
    }


}
