use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;
use rand_distr::{Normal, Distribution};
use std::fs::File;
use std::io::{self, Write};
use csv::Writer;
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

pub(crate) fn initialize_csv_writer(file_path: &str) -> Result<Writer<File>, io::Error> {
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["Day", "Goods Produced", "Total Money", "Technology"])?;
    Ok(wtr)
}

pub(crate) fn write_simulation_data(
    wtr: &mut Writer<File>,
    day: usize,
    goods_produced: &HashMap<String, i32>,
    total_money: f32,
    technology: f32,
) -> Result<(), csv::Error> {
    let goods_produced_str = format!("{:?}", goods_produced);
    wtr.write_record(&[
        day.to_string(),
        goods_produced_str,
        total_money.to_string(),
        technology.to_string(),
    ])?;
    wtr.flush()?;
    Ok(())
}