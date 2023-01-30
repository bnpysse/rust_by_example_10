// 2023年1月30日23时32分4秒，模块学习
//regin 10.模块
// 10.1.可见性
// 默认情况下，模块中的项拥有私有的可见性(private visibility)，不过可以加上 pub
// 修饰语来重载这一行为。模块中只有公有的 (public) 可以从模块外的作用域访问。
mod my_mod {
    fn private_function() {
        println!("called 'my_mod::private_function()'");
    }
    pub fn function() {
        println!("called 'my_mod::function()'");
    }
    // 在同一模块中，项可以访问其它项，即使它是私有的
    pub fn indirect_access() {
        print!("called 'my_mod::indirect_access()', that\n");
        private_function();
    }
    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called 'my_mod::nested::function()'");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called 'my_mod::nested::private_function()'");
        }
        // 使用 'pub(in path) 语法定义的函数只在给定的路径中可见
        // 'path' 必须是父模块(parent module)或祖先模块(ancestor module)
        pub(in crate::my_mod)  fn public_function_in_my_mod() {
            print!("called 'my_mod::nested::public_function_in_my_mod()', that\n");
            public_function_in_nested();
        }
        // 使用 'pub(self)' 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called 'my_mod::nested::public_function_in_nested'");
        }
        // 使用 'pub(super)' 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }
    pub fn call_public_in_my_mod() {
        print!("called 'my_mod::call_public_function_in_my_mod()', that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }
    // 'pub(crate)' 使得函数只在当前 crate 可见
    pub(crate) fn public_function_in_crate() {
        println!("called 'my_mod::public_function_in_crate()'");
    }
    // 嵌套模块中的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called 'my_mod::private_nested::function()'");
        }
    }
}