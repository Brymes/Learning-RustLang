// area of triangle = 1/2 * base * height
#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

fn area(triangle: &Triangle) -> f64 {
    triangle.base * triangle.height * 0.5
}

fn main() {
    let a_triangle = &Triangle {
        base: 7.0,
        height: 20.0,
    };

    let calc_area = area(a_triangle);

    println!("the area of the triangle is {} square cm", calc_area);

    // you can't print out struct values in placeholders. You have to use debug syntax {:?} 
    // or get more insights with {:#?}
    println!("triangle 1: {:#?}", a_triangle);
    // you can also use the dbg! macro to output the standard error
    dbg!(&a_triangle);
}