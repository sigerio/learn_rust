fn main() {
/***********************************************************/
    /*immutable 不可变变量*/
    let immutable_x = 5;    
    println!("The value of immutable_x if : {immutable_x}");

    // immutable_x = 6;
    // println!("The value of x if : {immutable_x}");
/***********************************************************/
    /*增加 mut 来声明一个可变变量*/
    let mut uimmutable_x = 5;
    println!("The value of uimmutable_x if : {uimmutable_x}");

    uimmutable_x = 6;
    println!("The value of uimmutable_x if : {uimmutable_x}");
/**************************************************************/
    /*声明一个常量，该常量不可修改*/
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS if : {THREE_HOURS_IN_SECONDS}");

/***********************************************************/
    /*Shadowing 遮蔽，rust允许变量名重复，这意味着在后续的使用中，实际使用的是第二个变量的值，书面解释为第一个变量被第二个变量遮蔽了*/
    // let mut x = 5;
    let x = 5;

    let x = x + 1;
    //{}意味着将内部的区域和外部的区域隔离开，
    {
        let x = x * 2;
        println!("The value of x in the inner 0 scope is: {x}");
    }

    {
        let mut x = 99; //虽然可以增加mut修饰符，但是不建议这么做，因为会出现warning 
        println!("The value of x in the inner 1 scope is: {x}");

    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len(); //遮蔽甚至允许同样的变量名为不同的变量类型，比如第一个spaces是字符串型，第二个spaces是数字型

/*********************************************************************/
    /*元组*/
    let tup: (i32, u32, f32, u8) = (-500, 20, 3.2, 5);
    /*元组的结构*/
    let(x, y, z, p) = tup;

    println!("The value of y is: {y}");
    /*通过索引访问元组成员*/
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("The value of one is: {one}");

/*********************************************************************/
    /*数组*/
    let a = [1, 2, 3, 4, 5];
    /*具有5个i32类型元素的数组*/
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //你发现了吗，这里使用了shadowing :) 
    /**/
    let a = [3; 5]; //等价于let a = [3,3,3,3,3];

    /*尝试一下非法的越界访问数组 在编译阶段就报错辣 XD*/
    // let index = a[5];
    // println!("The value of index is: {index}");

    /*一个简单的越界访问的测试程序*/

    let a = [1, 2, 3, 4, 5];
    println!("************a simple process to test out of rang access******************");
    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()   //这种写法可以让我们不在头部声明use std::io 即可在程序内调用方法
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
