// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics -- DONE
// * Use an enum for the box color -- DONE
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// Instructor Solution

enum Color {
    Black,
    Green,
    Yellow,
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("Color: Black"),
            Color::Green => println!("Color: Green"),
            Color::Yellow => println!("Color: Yellow"),
            Color::Red => println!("Color: Red"),
        }
    }
}


struct ShippingBox {
    dimensions: Dimensions, // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<||
    color: Color, // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<|| ||
    weight: f64, // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<|| || ||
} //                                                                        || || ||
  //                                                                        || || ||
impl ShippingBox {      //                                                  || || || Reference back to the struct
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self { //  || || ||
        Self { //                                                           || || ||
            weight, // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>|| || ||
            color, // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>|| ||
            dimensions, // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>||
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}lbs", self.weight);
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}in", self.width);
        println!("Height: {:?}in", self.height);
        println!("Depth: {:?}in", self.depth);
    }
}

fn main() {
    println!("------------------");
    println!("Box Info:");
    
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    
    let small_box = ShippingBox::new(7.3, Color::Yellow, small_dimensions);
    small_box.print();
    
    println!("------------------");
    /*
    let small_box = ShippingBox {
        dimensions: Dimensions {
            width: 6.5,
            height: 8.0,
            depth: 10.0,
        },
        color: Color::Black,
        weight: 15.7,
    };
    */

    /*

    let red = Box::red_box();
    red.print_box_info();

    */

}
