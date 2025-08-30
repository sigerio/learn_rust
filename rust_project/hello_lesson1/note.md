1 我们在这里学到了简单的println! 的使用，这表示这将调用一个宏，而不是调用一个函数
2 我们学到了cargo的使用，分别使用了cargo new oroject 来创建一个rust工程，使用cargo build来编译生成可执行文件，而且遇到了permission denied错误，在运行了cargo clean之后，奇妙的好了；
  还学到了执行cargo run，这会直接编译工程并运行，当然这一切还都是debug
3 需要发布的时候执行 cargo build --release ，编译器会优化最后的可知那个文件