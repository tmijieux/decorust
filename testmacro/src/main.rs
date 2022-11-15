extern crate mymacro;
use mymacro::wrap_func;


struct Bidule;
impl Bidule {
    // #[wrap_func]
    fn myfunc(&self) {
        println!("Bidule myfunc !");
    }
}

//#[wrap_func]
pub fn youyou((a,b): (i32,i32), _c: i32) -> i32 {
    println!("youyou {}", a+b);
    6
}



#[wrap_func(Hello, Coucou)]
pub fn hello(a: i32, b: i32) -> i32 {
    #![inline]
    println!("hello {}", a+b);
    6
}

fn main() {

    #[wrap_func]
    fn lel() {
        println!("coucou")
    }
    lel();

    hello(5, 7);
    hello(5, 7);
    hello(5, 7);

    Bidule.myfunc();
    youyou((3,4),5);
    println!("Hello, world!");
}

