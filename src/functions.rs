use rand::Rng;
use rand::thread_rng;
use rand_distr::{Normal, Distribution};

pub(crate) struct NormalDist {
    normal: Normal<f64>,
}

impl NormalDist {
    pub(crate) fn new(mean: f64, std_dev: f64) -> Self {
        NormalDist {
            normal: Normal::new(mean, std_dev).unwrap(),
        }
    }

    pub fn sample(&self) -> f32 {
        let mut rng = thread_rng();
        self.normal.sample(&mut rng) as f32 
    }
}    