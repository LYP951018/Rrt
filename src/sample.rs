extern crate rand;

use basic::Vector2;
use sample::rand::Rng;

pub fn random(samples: u32) -> Vec<Vector2> {
    rand::thread_rng().gen_iter().take(samples as usize).collect()
}

pub fn jitter(samples: u32) -> Vec<Vector2> {
    let width = (samples as f32).sqrt() as u32;
    let widthf = width as f32;
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    for x in 0..width {      
        for y in 0..width {
            let dx = rng.gen_range(0.0, 1.0);
            let dy = rng.gen_range(0.0, 1.0);
            let xf = x as f32;
            let yf = y as f32;
            result.push(((xf + dx) / widthf, (yf + dy) / widthf));
        }
    }
    result
}

pub fn nrooks(samples: u32) -> Vec<Vector2> {
    

}