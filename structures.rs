#[derive(Debug)]
#[derive(Copy, Clone)]
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
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32
{
    let area: f32;
    let Rectangle { top_left: Point { x: tlx, y: tly }, bottom_right: Point { x: brx, y: bry } } = rect;
    area = (brx - tlx) * (bry - tly);
    return area;
}

fn get_peter() -> String
{
    return String::from("Peter");
}

fn main() {
    // Create struct with field init shorthand
    let name = get_peter();
    let mut other = name.clone();
    unsafe {
        let bytes = other.as_bytes_mut();
        bytes[2] = 0x32;
        println!("Other: {}", other);
    }
    let age = 27;
    let peter = Person { name, age };
    let Person{ name: newname, age: _newage } = peter;

    println!("New name: {}", newname);
    //println!("old name: {}", name);

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
    let Point { x: left_edge, y: top_edge } = point;

    let mut rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("Area: {}", rect_area(rectangle));

    rectangle = Rectangle {
        top_left: Point { x: 0.5, y: 1.0 },
        bottom_right: Point { x: 2.0, y: 2.5 },
    };

    println!("Area 2: {}", rect_area(rectangle));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
