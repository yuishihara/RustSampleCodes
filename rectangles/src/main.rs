#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        rectangle.width < self.width && rectangle.height < self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size
        }
    }
}

fn main() {
    let width1 = 50;
    let height1 = 50;

    println!(
        "The area of rectangle is {} square pixels",
        areaTwoArgs(height1, width1)
    );

    let rect1 = (width1, height1);
    println!(
        "The area of rectangle is {} square pixels",
        areaTuple(rect1)
    );

    let rect1 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area of rectangle is {} square pixels",
        areaStruct(&rect1)
    );

    println!("Rectangle is {:?}", rect1);

    rectangle_size();
    associated_functions();
}

fn associated_functions() {
    let size = 10;
    let rect = Rectangle::square(size);
    println!("rectangle width: {}, height: {}", rect.width, rect.height);
}

fn rectangle_size() {
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn areaTwoArgs(width: u32, height: u32) -> u32 {
    width * height
}

fn areaTuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn areaStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
