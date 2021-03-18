#![allow(dead_code)]

pub mod foo {
    pub fn public() {
        println!("Public function called");
        private();
    }

    fn private() {
        println!("Private function called");
    }
}

pub fn main() {
    //foo::public();
    //dbg!((foo::private as usize) - (foo::public as usize));
    
    // Calling a private function from outside the module
    unsafe {
        std::mem::transmute::<usize, fn() -> ()>((foo::public as usize) + 64)();
    }
}

