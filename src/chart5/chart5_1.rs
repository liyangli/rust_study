fn area(width: u32, height: u32) ->u32{
    width * height
}

pub fn main(){

    // let width2 = 30;
    // let height2 = 50;

    // println!("The area of the rectangle is {} square pixels.",
    //     area(width2, height2)
    // );

    // let rect = (10, 20);
    // println!("The area of the rectangle is {} square pixels.",
    //     area2(rect)
    // );

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30*scale),
        height: 20,
    };
    // println!("rect1 is {rect:?}")
    dbg!(&rect);
}

fn area2(dimensions: (u32, u32)) ->u32{
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

