// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(x: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: tlx, y: tly },
        bottom_right: Point { x: brx, y: bry },
    } = x;
    let width = brx - tlx;
    let height = bry - tly;
    println!("{}x{}", width, height);
    (width * height).abs()
}

fn square(top_left: Point, side: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: top_left.x,
            y: top_left.y,
        },
        bottom_right: Point {
            x: top_left.x + side,
            y: top_left.y + side,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!(
        "{}",
        rect_area(&Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 10.0, y: 5.0 }
        })
    );

    let sq = square(Point { x: 5.0, y: 7.5 }, 10.0);

    println!(
        "square bottom right ({}, {}), area = {}",
        sq.bottom_right.x,
        sq.bottom_right.y,
        rect_area(&sq)
    );
}
