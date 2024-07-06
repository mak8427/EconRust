mod actor;
mod market;
mod workplace;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use market::Market as OtherMarket;

use actor::Actor as OtherActor;
use rand::Rng;
use crate::workplace::Workplace;


fn main() {
    let n = 10;
    let Technology:f32 = 1.0;
    
    // Agents initialization
    
    let mut actor_1 = Rc::new(RefCell::new(OtherActor::new(rand::thread_rng().gen_range(0.0..50.0), "Test".into(), 20)));
    
    let mut market_1 = Rc::new(RefCell::new(OtherMarket::new()));
    let mut workplace_1 = Rc::new(RefCell::new(Workplace::new(HashMap::new(), "Test".into(), 1.0)));    // Market initialization
    market_1.borrow_mut().add_good(10.0, "Potatos".into());

    workplace_1.borrow_mut().add_worker(actor_1.clone());


    // Simulation Step
    let mut i = 0;
    while i < n {
        actor_1.borrow_mut().buy_needs(market_1.clone());
        actor_1.borrow_mut().increase_population(1);
        workplace_1.borrow_mut().produce();

        
        i += 1;
    }
    
}