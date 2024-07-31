use std::collections::HashMap;
use crate::actor::Actor;
use std::rc::Rc;
use std::cell::RefCell;
use crate::market::Market;
use log::{info};
pub struct Workplace {
    workers: Vec<Rc<RefCell<Actor>>>,
    name: String,
    money: f32,
    //the type of goods produced and the amount produced in total
    pub(crate) goods_produced: HashMap<String, i32>,
    
    technology: f32,
}

impl Workplace {
    pub fn new(mut goods_produced: HashMap<String, i32>, name: String, technology: f32) -> Workplace {
        if goods_produced.len() == 0 {
            goods_produced.insert("Potatoes".into(), 0);
        }

        Workplace { workers: Vec::new(), name, money: 0.0, goods_produced, technology }
    }

    //Add a worker to the workplace (points to the actor)
    pub fn add_worker(&mut self, worker: Rc<RefCell<Actor>>) {
        self.workers.push(worker);
    }

    //Produce goods
    pub fn produce(&mut self) {
        let mut effective_number_of_workers = 0;
        for worker in &self.workers {
            effective_number_of_workers += worker.borrow().population_val();
            info!("N of Workers: {}", worker.borrow().population_val());
        }

        info!("Effective number of workers: {}", effective_number_of_workers);
        let production: i32 = (effective_number_of_workers as f32 * self.technology).round() as i32;

        self.goods_produced.insert("Potatoes".into(), production);
    }


    pub fn sell_goods(&mut self, market: Rc<RefCell<Market>>) {
        for (key, value) in &self.goods_produced {
            market.borrow_mut().increase_q_sold(key.clone(), *value);
            info!("Selling {} to the market", key.clone());
        }
    }


    pub fn profit(&mut self, market: Rc<RefCell<Market>>) {
        for (key, value) in &self.goods_produced {
            self.money += market.borrow().get_good(key).unwrap().price * (*value as f32);
        }
    }
    
    pub fn pay_workers(&mut self) {
        for worker in &self.workers {
            worker.borrow_mut().get_paid(self.money / self.workers.len() as f32);
        }
    }
}