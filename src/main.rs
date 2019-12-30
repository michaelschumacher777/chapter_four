fn main() {
    println!(">>> Chapter Four");

    println!("----------------------------------------------------");
    page_66();
    println!("----------------------------------------------------");
    page_67();
    println!("----------------------------------------------------");
    references_and_borrowing();
    println!("----------------------------------------------------");

    println!("<<< Chapter Four");
}

fn references_and_borrowing() {
    println!(">>> references_and_borrowing");

    let s1 = String::from("Hello s1");

    let len = calculate_length(&s1);

    println!("Length of the s1({}) is {}", s1, len);

    println!("<<< references_and_borrowing");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn page_67() {
    println!(">>> page_67");
    let s1 = gives_ownership();
    println!("1 contents of s1:{}", s1);
    let s2 = String::from("Hello 2");
    println!("2 contents of s1:{}", s1);
    println!("2 contents of s2:{}", s2);

    let _s3 = takes_and_gives_back(s2);
    println!("3 contents of s1:{}", s1);

    println!("3 contents of s3:{}", _s3);
    println!("<<< page_67");
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello-gives");

    some_string
}

fn page_66() {
    println!(">>> Page_66 function");
    let my_string = String::from("This is the contents of the string - go figure");
    takes_ownership(my_string);
//    println!("Should error here {}", my_string);

    let x=5;
    makes_copy(x);

    println!("Value of x: {}", x);
    println!("<<< Page_66 function");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

