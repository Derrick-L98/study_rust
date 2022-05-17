pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("{}.",value)
        } 

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}




fn main() {
    println!("Hello, world!");
}

忽略测试，运行剩余测试
ignore属性
#[cfg(test)]
单独标记的测试
运行被忽略的测试
cargo test -- --ignore

rust 对测试的分类
    单元测试
    集成测试
    单元测试：
        小，专注
        一次对一个模块进行隔离的测试
        可测试private接口
    集成测试：
        在库外部。和其他外部代码一样使用你的代码
        只能使用public接口
        可能在每个测试中使用多个模块

单元测试：
    #[cfg(test)]标注
        tests模块上的test才编译和运行代码
        运行cargo build 

测试private函数
