use std::f64::consts::PI;

use rand::distributions::Distribution;
use rand::distributions::Uniform;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct MoonsDataset {
    pub points: Vec<Point>,
    pub labels: Vec<i32>,
}

pub fn generate_moons(n_samples: usize, noise: f64) -> MoonsDataset {
    let mut rng = rand::thread_rng();
    let noise_dist = Uniform::from(-noise..noise);

    let mut moons = MoonsDataset {
        points: Vec::new(),
        labels: Vec::new(),
    };

    for i in 0..n_samples {
        let angle = PI * (i as f64) / (n_samples as f64 / 2.0);
        let (dx, dy) = (noise_dist.sample(&mut rng), noise_dist.sample(&mut rng));

        if i % 2 == 0 {
            // First moon
            let point = Point {
                x: angle.sin() + dx,
                y: angle.cos() + dy,
            };
            moons.points.push(point);
            moons.labels.push(0);
        } else {
            // Second moon
            // moons.points.push((x, y));
            let point = Point {
                x: (1.0 + angle).sin() + dx,
                y: (1.0 - angle).cos() + dy,
            };
            moons.points.push(point);
            moons.labels.push(1);
        }
    }

    moons
}
