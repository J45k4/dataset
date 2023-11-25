use std::f64::consts::PI;

use rand::distributions::Distribution;
use rand::distributions::Uniform;

pub fn generate_moons(n_samples: usize, noise: f64) -> (Vec<(f64, f64)>, Vec<i32>) {
    let mut rng = rand::thread_rng();
    let noise_dist = Uniform::from(-noise..noise);

    let mut data = Vec::with_capacity(n_samples);
    let mut labels = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let angle = PI * (i as f64) / (n_samples as f64 / 2.0);
        let (dx, dy) = (noise_dist.sample(&mut rng), noise_dist.sample(&mut rng));

        if i % 2 == 0 {
            // First moon
            let x = angle.sin() + dx;
            let y = angle.cos() + dy;
            data.push((x, y));
            labels.push(0);
        } else {
            // Second moon
            let x = (1.0 + angle).sin() + dx;
            let y = (1.0 - angle).cos() + dy;
            data.push((x, y));
            labels.push(1);
        }
    }

    (data, labels)
}
