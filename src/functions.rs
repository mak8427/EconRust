use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;
use rand_distr::{Normal, Distribution};
use std::fs::File;
use std::io::{self, Write};
use std::rc::Rc;
use csv::Writer;
use crate::actor::Actor;
use crate::market::Market;
use crate::workplace::Workplace;

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
    wtr.write_record(&["Day", "Total Goods Produced", "Total Actors Money", "Technology", "Total Population", "Total Q_bought", "Total Q_sold"])?;
    Ok(wtr)
}

pub(crate) fn write_simulation_data(
    wtr: &mut Writer<File>,
    day: usize,
    workplaces: &Vec<Rc<RefCell<Workplace>>>,
    actors: &Vec<Rc<RefCell<Actor>>>,
    market: &Rc<RefCell<Market>>,
) -> Result<(), csv::Error> {
    let total_goods_produced: i32 = workplaces.iter().map(|w| w.borrow().goods_produced.values().sum::<i32>()).sum();
    let total_actors_money: f32 = actors.iter().map(|a| a.borrow().money).sum();
    let technology = workplaces[0].borrow().technology;
    let total_population: usize = actors.len();
    let total_q_bought: i32 = market.borrow().goods.iter().map(|g| g.q_bought).sum();
    let total_q_sold: i32 = market.borrow().goods.iter().map(|g| g.q_sold).sum();

    wtr.write_record(&[
        day.to_string(),
        total_goods_produced.to_string(),
        total_actors_money.to_string(),
        technology.to_string(),
        total_population.to_string(),
        total_q_bought.to_string(),
        total_q_sold.to_string(),
    ])?;
    wtr.flush()?;
    Ok(())
}