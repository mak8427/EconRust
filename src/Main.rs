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
use chrono::Local;
use fern::Dispatch;
use log::{info};
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
        // Log to file
        .chain(fern::log_file("output.log")?)
        // Also log to console
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logging().expect("Failed to initialize logging.");
    
    
    //variables 
    let n = 20;
    let number_of_agents = 10;
    let Technology:f32 = 0.8;
    
    // Agents initialization
    //initialise and array of actors
    let mut actors = Vec::new();
    while actors.len() < number_of_agents {
        actors.push(Rc::new(RefCell::new(OtherActor::new(rand::thread_rng().gen_range(1000.0..5000.0), "Test".into(), rand::thread_rng().gen_range(1..20)))));
    }
    
    
    
    
    let mut market_1 = Rc::new(RefCell::new(OtherMarket::new()));
    market_1.borrow_mut().add_good(10.0, "Potatoes".into());

    // Workplace initialization
    let mut workplace_1 = Rc::new(RefCell::new(Workplace::new(HashMap::new(), "Test".into(), Technology)));    // Market initialization
    for actor in &actors {
        workplace_1.borrow_mut().add_worker(actor.clone());
    }
    // Simulation Step
    let mut i = 0;
    while i < n {
        info!("Day: {}", i);
        

        
        
        workplace_1.borrow_mut().produce();
        workplace_1.borrow_mut().sell_goods(market_1.clone());


        for actor in &actors {
            actor.borrow_mut().buy_needs(market_1.clone());
            actor.borrow_mut().increase_population(1);
        }
        
        market_1.borrow_mut().update_good_price();
        market_1.borrow_mut().new_day();

        
        i += 1;
    }
    
    println!("END OF SIMULATION");
}