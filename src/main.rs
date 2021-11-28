use std::io;

fn main() {
    println!("Please input your a parameter");
    let a = get_input();

    println!("Please input your b parameter");
    let b = get_input();

    println!("Please input your c parameter");
    let c = get_input();

    println!(
        "Params: 
    a = {}
    b = {}
    c = {}
    ",
        a, b, c
    );

    if a == 0.0 {
        println!("The function is linear!");
        println!("x = {}", -c / b);
        return;
    }

    let delta = get_delta(a, b, c);

    if delta == 0.0 {
        println!("Delta equals zero!");
        println!("x = {}", (-b) / (2.0 * a));
        return;
    }

    if delta < 0.0 {
        println!("Delta less than 0! No valid x found.");
        return;
    }

    let (x_1, x_2) = get_x(a, b, delta);

    println!("x1 = {}", x_1);
    println!("x2 = {}", x_2);
}

fn get_input() -> f32 {
    loop {
        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("kek");

        match num.trim().parse::<f32>() {
            Ok(num) => {
                break num;
            }
            Err(..) => println!("Input a valid number"),
        };
    }
}

fn get_delta(a: f32, b: f32, c: f32) -> f32 {
    b.powf(2.0) - 4.0 * a * c
}

fn get_x(a: f32, b: f32, delta: f32) -> (f32, f32) {
    let x_1 = (-b - delta.sqrt()) / (2.0 * a);
    let x_2 = (-b + delta.sqrt()) / (2.0 * a);

    (x_1, x_2)
}
