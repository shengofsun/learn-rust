mod my_module {
    fn private_function() {
        println!("called my_module::private_function");
    }

    pub fn pub_function() {
        println!("called my::pub");
    }

    pub fn indirect_access() {
        println!("indirect access to functions");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("my::nested::function");
        }
    }
}
