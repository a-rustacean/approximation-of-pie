use rand::{Rng, thread_rng};

fn len(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

fn main() {
    let total_points = 1_000_000_0;
    let mut points_in_circle = 0;
    let mut rng = thread_rng();
    
    for _ in 0..total_points {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let dist = len(x, y);
        if dist < 1.0 {
            points_in_circle += 1;
        }
    }
    println!("{}", points_in_circle as f32 * 4.0 / total_points as f32);
}
