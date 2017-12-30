extern crate rand;

use self::rand::Rng;
use vectors::Vector2;

pub fn random<T: Rng>(rng: &mut T, samples: u32) -> Vec<Vector2> {
    rng.gen_iter().take(samples as usize).collect()
}

pub fn jitter<T: Rng>(rng: &mut T, result: &mut Vec<Vector2>, samples: u32) {
    result.clear();
    let width = (samples as f32).sqrt() as u32;
    let widthf = width as f32;
    for x in 0..width {
        for y in 0..width {
            let dx = rng.gen_range(0.0, 1.0);
            let dy = rng.gen_range(0.0, 1.0);
            let xf = x as f32;
            let yf = y as f32;
            result.push(Vector2::new((xf + dx) / widthf, (yf + dy) / widthf));
        }
    }
}

pub fn nrooks<T: Rng>(rng: &mut T, result: &mut Vec<Vector2>, samples: u32) {
    result.clear();
    let sf = samples as f32;
    for i in 0..samples {
        let i = i as f32;
        let x = (i + rng.gen_range(0.0, 1.0)) / sf;
        let y = (i + rng.gen_range(0.0, 1.0)) / sf;
        result.push(Vector2::new(x, y));
    }
    rng.shuffle(result);
}

pub fn box_filter(samples: &mut Vec<Vector2>) {
    for sample in samples {
        sample.x -= 0.5;
        sample.y -= 0.5;
    }
}
