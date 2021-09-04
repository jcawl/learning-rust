struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    //one way to get area
    //can be simplified
    let w = 15;
    let h = 10;
    let area1 = two_param_area(w,h);
    println!("area1: {}", area1);

    //refactoring with a tuple
    let wh = (10, 15);
    let area2 = tuple_area(wh);
    println!("area2: {}", area2);

    //refactoring with struct
    //most verbose and easiest to understand
    let rect = Rectangle {
        width: 15,
        height: 10
    };
    //pass struct as reference so main retains ownership
    let area3 = struct_area(&rect);
    println!("area3: {}", area3);


}

fn two_param_area(w: u32, h: u32) -> u32 {
    h * w
}

fn tuple_area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn struct_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}