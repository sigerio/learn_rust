fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);
    print_labeled_measurement(3,'h');




    /**语句和表达式**/
    let y = 6; // 这是一个语句，准确点是赋值语句
    println!("The value of y is: {y}");

    let y =         //而这里， {};  所包含的是一个表达式，表达式结尾没有；  如果有；则变成了一个语句了
    {
        let x = 3;
        x+1
    };
    println!("The value of y is: {y}");


    /*带返回值的函数*/
    let x = five();
    println!("The value of x is: {x}");


    let x = plus_one(5);

    println!("The value of x is: {x}");
}



/*不幸的是，函数不支持遮蔽 :)*/
//rust不关心被调函数的定义位置
fn another_function() {
    println!("Another function.");
}


fn another_function1(x: i32) {
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


/*具有返回值的函数*/
fn five() -> i32{
    5
}

//TODO change x+1; to x+1
fn plus_one(x: i32) -> i32 {
    x + 1; //加了;之后，表示x+1是一个语句，而语句不会返回任何值，函数的声明表示该函数将要返回一个i32的值，所以引起错误，
}

