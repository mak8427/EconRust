use std::collections::HashMap;
use crate::actor::Actor;

pub struct Workplace {
    workers: Vec<*mut Actor>,
    name: String,
    //the type of goods produced and the amount produced in total
    goods_produced: HashMap<String, f32>,
}

impl Workplace {
    pub fn new(mut goods_produced: HashMap<String, f32>, name: String ) -> Workplace {
        
        if goods_produced.len() == 0 {
            goods_produced.insert("Potatos".into(), 0.0);
        }

        Workplace { workers: Vec::new(), name: "Test".into(), goods_produced }
    }
    
    //Add a worker to the workplace (points to the actor)
    pub fn add_worker(&mut self, worker: *mut Actor) {
        self.workers.push(worker);
    }
    
    
    
    

}
