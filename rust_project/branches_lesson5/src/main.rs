fn main() {

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    /*在语句中使用if表达式*/
    let y = true;
    // let x = if y { 5 } else { "five" };  //这个时候你会发现编译报错，为什么呢，还记得我们说过rust是静态类型语言吗，所有变量在编译的时候就必须明确其类型！！！
    let x = if y { 5 } else { 6 };


    println!("The value of x is: {x}");


/*****************************************/

    /*loop*/
    loop{
        println!("loop again!");
        break;
    }

    /*loop 甚至可以返回一个参数！*/
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    /*使用标签来跳出指定的loop*/
    let mut count = 0;
    'counting_up: loop {    //为最外层的loop创建一个标签，counting_up
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;     //指示跳出counting_up这个标签指示的loop循环
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");





/****************************************/
    /*while*/
    let mut m = 3;
    while m != 0
    {
        println!("{m}");
        m -= 1;
    }
    println!("LIFTOFF!");


/****************************************/
/*for*/

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


    for element in a {
        println!("the value is: {element}");
    }


    for number in (1..4).rev() {    //rev功能可以用来反转数组，相当于倒着遍历
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
