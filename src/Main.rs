mod Actor;
mod Market;
use Market::Market as OtherMarket;

use Actor::Actor as OtherActor;
use rand::Rng;

fn main() {
    let N= 10;
    
    // Agents initialization
    let mut actor_1 = OtherActor::new(rand::thread_rng().gen_range(0.0..50.0), "Test".into());
    let mut market_1 = OtherMarket::new();
    
    
    
    // Simulation Step
    let mut i = 0;
    while i < N {
        
        i += 1;
    }
    
}