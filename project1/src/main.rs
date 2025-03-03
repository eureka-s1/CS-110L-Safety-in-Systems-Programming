
#[allow(unused)]
mod front_of_house {


    use crate::front_of_house::hosting::*;
    use crate::front_of_house::hosting::inner::*;
    mod hosting {
        pub fn add_to_waitlist() -> u32 {
            42
        }

        pub mod inner {
            pub fn another_func() {
                println!("Another function in the hosting module");
            }
        }
    }

    fn restaurant() {
        add_to_waitlist();
        another_func();
    }
    mod serving {
        use crate::front_of_house::hosting::*;
        fn take_order() {
            add_to_waitlist();
        }
        fn serve_order() {}
    }

}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;


#[allow(unused)]
fn main() {
    let x = 5;
    let y = 10;
    
    // 不可变引用
    let print_x = || println!("x: {}", x);
    
    // 可变引用
    let mut z = 15;
    let add_to_z = || {
        z += y;
        println!("z: {}", z);
    };
        
    let s = String::from("hello");
    // 值捕获
    let consume_s = move || {
        println!("s: {}", s);
    };

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    
    let v1_iter = v1.iter();

    let v3: Vec<i32> = v1.iter()
        .map(|x| *x + 1)
        .collect();
    
    println!("v1_iter: {:?}", v1_iter);
    println!("v3: {:?}", v3);
    
    // println!("s: {}", s);// 这句话会报错！即使并没有使用闭包，但 y 所有权已经在闭包内

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
