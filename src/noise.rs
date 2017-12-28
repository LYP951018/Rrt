use basic::Vector3;

lazy_static! {
    static ref PRECOMPUTED : [Vector3; 16] = {
        [
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(-1.0, 1.0, 0.0),
            Vector3::new(1.0, -1.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 1.0),
            Vector3::new(0.0, -1.0, 1.0),
            Vector3::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, -1.0, -1.0),
            Vector3::new(1.0, 1.0, 0.0),
            Vector3::new(-1.0, 1.0, 0.0),
            Vector3::new(0.0, -1.0, 1.0),
            Vector3::new(0.0, -1.0, -1.0)
        ]
    };
}


struct NoiseGenerator {}
