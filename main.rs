use rand::Rng:

fn len(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt();
}

fn main() {
    let totalPoints = 1_000_000_0;
    let mut pointsInCircle = 0;
    
    let mut i = 0;
    while i < totalPoints {
        i++;
        let x: f32 = Rng.gen();
        let y: f32 = Rng.gen();
        let dist = len(x, y);
        if (dist < 1) {
            pointsInCircle += 1;
        }
    }
    println!("{}", pointsInCircle * 4.0 / totalPoints);
}
