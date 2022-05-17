use std::thread;
use std::time::Duration;

struct Cacher<T>
where T: Fn(u32) -> u32
{
    	calculation: T,//闭包
        value: Option<u32>
}


impl<T> Cacher<T>
where 
    T:Fn(u32) -> u32,
{
    //实现关联构造函数
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }

    //实现一个方法：目的是使用value时，如果它为None，就执行这个闭包然后把结果存到value返回，如果不为None，直接返回
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {//判断value是否有值
            Some(v) => v,//有：返回
            None => {
                let v = (self.calculation)(arg);//执行闭包
                self.value = Some(v);//执行结果封装到Some里，赋值给value
                v//返回
            }
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

//模拟复杂算法
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ... ");
    thread::sleep(Duration::from_secs(2));//睡眠2秒
    intensity
}

//
//参数：强度、随机数
fn generate_workout(intensity: u32, random_number: u32) {

    //定义了一个匿名函数，然后把匿名函数的定义放入变量中，不是执行结果
    //没有使用前会报错，因为推断不出num的类型，一旦推断类型，就不能改变
    // let expensive_closure = |num| {
    //     println!("calculating slowly ... ");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_closure = Cacher::new(|num| {
            println!("calculating slowly ... ");
            thread::sleep(Duration::from_secs(2));
            num
        });

    if intensity < 25 {
        //1、两次调用了闭包，也就花费了两倍时间，解决方法（将闭包调用的执行结果存到本地变量，缺点：代码可能会重复）
        //第二中解决方法：创建一个struct，它持有闭包及其调用结果，只有在需要的时候才执行该结果。可以缓存结果
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            //不会调用匿名函数，也就不会浪费时间
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Totay, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}


