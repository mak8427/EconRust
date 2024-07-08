mod actor;
mod market;
mod workplace;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use market::Market as OtherMarket;
use log::{info, warn};
use actor::Actor as OtherActor;
use rand::Rng;
use crate::workplace::Workplace;
use chrono::Local;
use fern::Dispatch;

fn setup_logging() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logging().expect("Failed to initialize logging.");
    let n = 10;
    let Technology:f32 = 2.0;
    
    // Agents initialization
    
    let mut actor_1 = Rc::new(RefCell::new(OtherActor::new(rand::thread_rng().gen_range(0.0..50.0), "Test".into(), 20)));
    
    let mut market_1 = Rc::new(RefCell::new(OtherMarket::new()));
    let mut workplace_1 = Rc::new(RefCell::new(Workplace::new(HashMap::new(), "Test".into(), Technology)));    // Market initialization
    market_1.borrow_mut().add_good(10.0, "Potatoes".into());

    workplace_1.borrow_mut().add_worker(actor_1.clone());


    // Simulation Step
    let mut i = 0;
    while i < n {
        actor_1.borrow_mut().buy_needs(market_1.clone());
        actor_1.borrow_mut().increase_population(1);
        workplace_1.borrow_mut().produce();
        workplace_1.borrow_mut().sell_goods(market_1.clone());

        
        i += 1;
    }
    
    println!("END OF SIMULATION");
}