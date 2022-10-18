mod geometry;
mod scene;

fn main() {
    let y = {
        let x = 12;
        x + 1
    };
    for a in 1..3 {
        println!("{} Hello, world! {}", a, y);
    }
}
