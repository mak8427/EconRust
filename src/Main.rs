mod actor;
mod market;
mod workplace;
mod functions;

use std::fs::File;
use std::io::{self, Write};
use csv::Writer;
use rand::thread_rng;
use rand_distr::{Normal, Distribution};
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

    // Variables
    let n = 500;
    let number_of_agents = 5;
    let technology: f32 = 1.0;
    let growth_rate = 0.05;

    // Agents initialization
    let mut actors = Vec::new();
    while actors.len() < number_of_agents {
        actors.push(Rc::new(RefCell::new(OtherActor::new(
            rand::thread_rng().gen_range(1000.0..5000.0),
            "Test".into(),
            rand::thread_rng().gen_range(1..20),
            growth_rate,
        ))));
    }

    // Distribution Init
    let normal_dist = functions::NormalDist::new(1.0, 1.0);

    // Market initialization
    let mut market_1 = Rc::new(RefCell::new(OtherMarket::new()));
    market_1.borrow_mut().add_good(10.0, "Potatoes".into());

    // Workplace initialization
    let mut workplace_1 = Rc::new(RefCell::new(Workplace::new(
        HashMap::new(),
        "Test".into(),
        technology,
    )));
    for actor in &actors {
        workplace_1.borrow_mut().add_worker(actor.clone());
    }

    // Initialize CSV writer
    let mut csv_writer = functions::initialize_csv_writer("simulation_data.csv").expect("Failed to create CSV writer");

    // Simulation Step
    let mut i = 0;
    while i < n {
        info!("======= START DAY {} =======", i);

        workplace_1.borrow_mut().produce();
        workplace_1.borrow_mut().sell_goods(market_1.clone());
        workplace_1.borrow_mut().profit(market_1.clone());
        workplace_1.borrow_mut().pay_workers();

        for actor in &actors {
            actor.borrow_mut().buy_needs(market_1.clone());
            actor.borrow_mut().population_growth();
        }

        market_1.borrow_mut().update_good_price();
        market_1.borrow_mut().new_day();

        info!("Goods produced: {:?}", workplace_1.borrow().goods_produced);

        // Write data to CSV
        functions::write_simulation_data(
            &mut csv_writer,
            i,
            &workplace_1.borrow().goods_produced,
            workplace_1.borrow().money,
            workplace_1.borrow().technology,
        ).expect("Failed to write data to CSV");

        i += 1;
        workplace_1.borrow_mut().technology = rand::thread_rng().gen_range(0.8..1.2);

        info!("======= END DAY {} =======", i);
    }

    println!("END OF SIMULATION");
}