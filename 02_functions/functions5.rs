

fn main() {
    let answer: i32 = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}

// added a return to return the result of num * num