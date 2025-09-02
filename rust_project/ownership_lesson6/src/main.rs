fn main() {
    learn_string();
    drop_twice();
    drop_immediately();
    sclone();
    // println!("{s}");
    let x0 = 6;
    let y0 = x0;
    println!("y0  x0 {y0}  {x0}");


/******************************************************************************************/
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以s到这里不再有效
    // println!("{s}");             //there will be err when complite,because s is moved to function,so in this area, s has been released
    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
    println!("{}", x);              // 所以在后面可继续使用 x

/******************************************************************************************/

    let s1 = gives_ownership();        // gives_ownership 将它的返回值传递给 s1

    let s2 = String::from("hello");    // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被传入 takes_and_gives_back, 
                                       // 它的返回值又传递给 s3


    
}   // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 此处，s3 移出作用域并被丢弃。s2 被 move，所以无事发生
    // s1 移出作用域并被丢弃

/*{}表示的就是一个作用域，在作用域内的，那就是在作用域内的，和作用域外是两个东西，出了作用域之后，作用域内的东西全都作废*/

fn learn_string()
{
    let mut s = String::from("hello! string");  //String类型其实是在堆上分配了空间，这意味着在进入作用域时，自动分配了一部分大小的堆，在离开作用域后自动释放了空间
    s.push_str(" world!"); //在s后面追加字符串
    println!("{s}");
}


fn drop_twice()
{
    let s1 = String::from("hello");
    // let s2 = s1;         // after let s2 = s1, compiler think s1 is not valued at all,  this operate will make the heap where to store the "hello" be released now, and when leave this scope, drop again ,so compiler will prompt an error
    let _s2 = s1.clone();    //look at this function, this let data move to s2 when s1 be released 
    println!("{s1}, world!");   

}

fn drop_immediately()
{
    let mut s = String::from("hello");
    s = String::from("ahoy"); //when replace the value of s, heap where store "hello" will be drop immediately

    println!("{s}, world!");

}

fn sclone()
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

}

/*interim summary   drop will occured on heap ,not stack ,learn more to confirm this    */

/******************************************************************************************/
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处
/******************************************************************************************/








/******************************************************************************************/



fn gives_ownership() -> String {       // gives_ownership 将会把返回值传入
                                       // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string                        // 返回 some_string 并将其移至调用函数
}

// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}



/******************************************************************************************/


