mod actor;
mod market;
mod workplace;

use std::collections::HashMap;
use market::Market as OtherMarket;

use actor::Actor as OtherActor;
use rand::Rng;



fn main() {
    let n = 10;
    
    // Agents initialization
    
    let mut actor_1 = OtherActor::new(rand::thread_rng().gen_range(0.0..50.0), "Test".into(), 20);
    let mut market_1 = OtherMarket::new();
    let mut workplace_1 = workplace::Workplace::new(HashMap::new(), "Test".into());
    // Market initialization
    market_1.add_good(10.0, "Potatos".into());
    
    
    
    // Simulation Step
    let mut i = 0;
    while i < n {
        actor_1.buy_needs(&market_1);
        i += 1;
    }
    
}