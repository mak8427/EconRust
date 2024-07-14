use std::collections::HashMap;
use crate::actor::Actor;
use std::rc::Rc;
use std::cell::RefCell;
use crate::market::Market;
use log::{info};
pub struct Workplace {
    workers: Vec<Rc<RefCell<Actor>>>,
    name: String,
    //the type of goods produced and the amount produced in total
    goods_produced: HashMap<String, f32>,
    technology: f32,
}

impl Workplace {
    pub fn new(mut goods_produced: HashMap<String, f32>, name: String, technology: f32) -> Workplace {

        if goods_produced.len() == 0 {
            goods_produced.insert("Potatoes".into(), 0.0);
        }

        Workplace { workers: Vec::new(), name, goods_produced , technology}
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
        let production = effective_number_of_workers as f32 * self.technology;
        
        self.goods_produced.insert("Potatoes".into(), production);
    }
    
    
    
    pub fn sell_goods(&mut self, market: Rc<RefCell<Market>>) {
        for (key, value) in &self.goods_produced {
            market.borrow_mut().increase_q_sold(key.clone(), *value);
            info!("Selling {} to the market", key.clone()) ;
           
        }
    }
    
    //TODO: Computing profit and paying workers
}
