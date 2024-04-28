

fn main() {
    let a = ['Z','3', 'U', 'B', 'I', '1', '2', '3'];

    if a.len() >= 5 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
