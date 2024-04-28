// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

// match statement attempts to destructure y, referencing an Option<Point> type, trying to match Some(ref p) pattern, a reference to a Point type.
// When 'y' matches Some(ref p) pattern, it moves the "Point" struct out of Option type, binding it to 'p' variable.
// With ref p, instead of moving 'Point' struct, it creates a reference to 'Point' struct and binds it to 'p', maintaining ownership with 'y', making 'y' usable after match statement.
