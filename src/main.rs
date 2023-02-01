//region 10.1.模块可见性
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
        // #[allow(dead_code)]
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
        // #[allow(dead_code)]
        pub fn function() {
            println!("called 'my_mod::private_nested::function()'");
        }
    }
}
//endregion

//region 10.2.结构体的可见性
// 结构体的字段也是一个可见性的层次。字段默认拥有私有的可见性，也可以加上 pub 修饰语来重载该行为。
// 只有从结构体被定义的模块之外访问其字段时，这个可见性才会起作用，其意义是隐藏信息(即封装，encapsulation)
mod my {
    // 一个公有的结构体，带有一个公有的字段
    pub struct OpenBox<T> {
        pub contents: T,
    }
    pub struct ClosedBox<T> {
        pub contents: T,
    }
    // 一个公有的结构体，带有一个私有的字段
    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}
//endregion

//region 10.3.use声明
// use 声明可以将一个完整的路径绑定到一个新的名字
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()")
        }
    }
}
//endregion

//region 10.3.super和self
mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod mymy {
    fn function() {
        println!("called `my::function()`");
    }
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    pub fn indirect_call() {
        // 让我们从这作用域中访问所有名为 'function' 的函数！
        println!("called `my::indirec_call()`, that\n");

        // self 关键字表示当前的模块作用域，在这个例子里是 `my`。
        // 调用 `self::function()` 和直接调用 `function()` 的结果是一样的！
        // 因为它们表示相同的函数。
        self::function();
        function();

        // 我们也可以用 `self` 来访问 `my` 内部的另一个模块。
        se
    }
}
//endregion
fn main() {
    //region 10.1.模块可见性
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();
    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问。
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块内访问
    // my_mod::nested::public_function_in_my_mod();

    // 模块的私有而不能直接访问，即使它是嵌套在公有模块内部的
    // my_mod::private_function();

    // my_mod::nested::private_function();
    // my_mod::private_nested::function();
    //endregion

    //region 10.2.结构体的可见性
    println!("\n\n=====10.2.结构体的可见性=====");
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox {
        contents: "public information"
    };
    // 并且它们的字段可以正常访问到
    println!("The open box contains: {}", open_box.contents);
    let _closed_box = my::ClosedBox::new("classified information");
    //endregion

    //region 10.3.use声明
    println!("\n\n=====10.3.use声明=====");
    other_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        println!("Leaving block");
    }
    function();
    //endregion

    //region 10.3.super和self
    println!("\n\n=====10.3.super和self=====");
    // 可以在路径中使用 super(父级) 和 sef(自身) 关键字，从而在访问项时消除歧义，以及防止不必要的路径硬编码。


    //endregion
}
