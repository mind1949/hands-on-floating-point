fn main() {
    let x = float::Float::new(0.0006);
    let y = float::Float::new(0.0475);
    println!("{} + {} = {}", x, y, x + y);
    let x = float::Float::new(0.1);
    let y = float::Float::new(0.2);
    println!("{} + {} = {}", x, y, x + y);
    println!("{} - {} = {}", x, y, x - y);
}
