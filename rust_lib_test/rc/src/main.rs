// // // // enum list {
// // // //     Cons(i32, Box<List>),
// // // //     Nil,
// // // // }

// // // enum List {
// // //     Cons(i32, Rc<List>),
// // //     Nil,
// // // }
// // // use crate::List::{Cons, Nil};
// // // use std::rc::Rc;

// // // fn main() {
// // //     //通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会照成数据竞争，和不一致  

// // //     {
// // //         //let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
// // //         let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

// // //         //let b = Cons(3, Box::new(a));
// // //         let b = Cons(3, Rc::clone(&a)); //1
// // //         //let b = Cons(3, a.clone());     //2

// // //         //let c = Cons(4, Box::new(a));
// // //         let c = Cons(4, Rc::clone(&a));
// // //     }

// // //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// // //     println!("count after creating a = {}", Rc::strong_count(&a));  //1

// // //     let b = Cons(3, Rc::clone(&a));
// // //     println!("count after bind to b, a = {}", Rc::strong_count(&a));    //2

// // //     {
// // //         let c = Cons(4, Rc::clone(&a));
// // //         println!("count after bind c, a = {}", Rc::strong_count(&a));   //3
// // //     }

// // //     println!("count at end a = {}", Rc::strong_count(&a));  //2


// // //     println!("Hello, world!");
// // // }
// // #[derive(Debug)]
// // enum List {
// //     Cons(Rc<RefCell<i32>>, Rc<List>),
// //     Nil,
// // }
// // use crate::List::{Cons, Nil};
// // use std::rc::Rc;
// // use std::cell::RefCell;

// // fn main() {
// //     let value = Rc::new(RefCell::new(5));
// //     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
// //     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
// //     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
// //     println!("a={:?}", a);
// //     println!("b={:?}", b);
// //     println!("c={:?}", c);
// //     println!("+++++++++++++++++");

// //     *value.borrow_mut() += 10;
// //     println!("a={:?}", a);
// //     println!("b={:?}", b);
// //     println!("c={:?}", c);
// // }

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// use std::rc::Rc;
// use std::cell::RefCell;

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {//逻辑：方便访问Cons中第二个元素
//         match self {
//             Cons(_, item) =>Some(item),
//             Nil =>None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//     println!("a count = {}",Rc::strong_count(&a));//强引用个数--------------1
//     println!("a next item = {:?}",a.tail());//下一个元素
    
//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     println!("a count = {}",Rc::strong_count(&a));//---------------------2
//     println!("b count = {}",Rc::strong_count(&b));//---------------------1
//     println!("b next item = {:?}",b.tail());
    
//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);//a的第二个元素取出来指向b
//     }
//     println!("b count = {}",Rc::strong_count(&b));//--------------2
//     println!("b count = {}",Rc::strong_count(&a));//----------------2
    
//     println!("a next item = {:?}",a.tail());//打印a中的第二个元素即是b，而b的第二个元素是a，照成循环，导致溢出
// }

//创建一棵树：所有的节点都可以指向它的父节点和子节点
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,//避免循环引用
    children:RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());//.borrow()方法获得不可变引用，upgrade()方法尝试转换Vec<T>转换Rc<T>
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}