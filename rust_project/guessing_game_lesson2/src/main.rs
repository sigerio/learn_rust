use std::cmp::Ordering;

use std::io; //使用标准(std)的输入输出(io)库

use rand::Rng; //使用rand库的Rng接口



fn main() {
    println!("Guss the Number!");
    println!("Hi, Input your guess!");

    // let mut guess = String::new();

        

    let secret_number = rand::thread_rng().gen_range(1..=100); //创建一个1-100的随机数secret_number ，并且不可修改
    // println!("The secret number is: {secret_number}");

    loop{
        //这里有个什么坑呢，当guess在loop外创建后，每次赋值并不会清楚之前的值，意味着当你连续输入两次50，guess的值并不是50，而是50\n50，从而引起expect异常

        let mut guess = String::new(); // 这里的let是一个变量修饰符， 如果是let guess = 5；则guess将是一个不可修改的值， 但是let mut guess = 5； 这意味着guess可以被重新赋值，接下来的String::new意味着这里创建的guess是一个String类返回的实例

            
        io::stdin()
            .read_line(&mut guess) //这两行是读取用户的输入，并使用read_line方法将内容存储在guess中， 注意这里使用了mut guess，这意味着这将是一个整体，
            .expect("Failed to read line"); //错误处理的方法，用于处理read_line的返回结果，一旦出现问题，就将调用该方法

        
            // let guess: u32 = guess.trim().parse().expect("Please type a number!"); //复用guess这个变量名，在后面会详细介绍。trim方法会消除空白字符 parse会将字符串转成其他类型,具体类型由: U32来指定
        let guess: u32 = match guess.trim().parse() { //使用match来对parse的返回结果进行处理，从而摆脱expect导致的程序结束
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You gaussed: {guess}"); //有意思的是，这里不像是c语言，需要指定变量类型，而是直接使用{}来修饰需要输出的变量，{guess}意味着将直接打印guess的值


        match guess.cmp(&secret_number) {                   //还记得guess的类型吗，它是一个String类的实例，并且在前面赋了值 这一步会比较guess和secret_number，同时根据返回的Ordering来判断该如何执行下一步
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{ 
                println!("You win!");
                break;
            }
        }
    }
    //test line

    // let a = 1;
    // let b = 2;
    // let c = a+b;
    // println!("{a} + {b} = {}",c);//你注意到了吗，你甚至不用输入 \n  就能在打印的时候换行！
}
