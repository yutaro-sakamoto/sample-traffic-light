mod light;

use light::Light;

fn main() {
    println!("Yellow 1: {:?}", Light::Yellow(1));
    println!("Blue 2: {:?}", Light::Blue(2));
    println!("Red 3: {:?}", Light::Red(3));
}
