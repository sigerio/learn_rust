fn main() {
    learn_string();
    drop_twice();
    drop_immediately();
    sclone();
    // println!("{s}");




    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
    println!("{}", x);              // 所以在后面可继续使用 x


    
}

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




