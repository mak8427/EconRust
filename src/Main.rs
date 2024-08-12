mod actor;
mod market;
mod workplace;
mod functions;
mod SimulationApp;

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
use plotters::prelude::*;
use plotters::style::full_palette::BLUE;

fn main() {
    functions::setup_logging().expect("Failed to initialize logging.");

    // Variables
    let n = 500;
    let number_of_agents = 5;
    let number_of_workplaces = 1;
    let technology: f32 = 1.0;
    let growth_rate = 0.05;

    // Agents initialization
    let mut actors = Vec::new();
    let mut workplaces = Vec::new();

    while actors.len() < number_of_agents {
        actors.push(Rc::new(RefCell::new(OtherActor::new(
            rand::thread_rng().gen_range(1000.0..5000.0),
            "Test".into(),
            rand::thread_rng().gen_range(1..20),
            growth_rate,
        ))));
    }

    while workplaces.len() < number_of_workplaces {
        workplaces.push(Rc::new(RefCell::new(Workplace::new(
            HashMap::new(),
            "Test".into(),
            technology,
        ))));
    }

    // Distribution Init
    let normal_dist = functions::NormalDist::new(1.0, 1.0);

    // Market initialization
    let mut market_1 = Rc::new(RefCell::new(OtherMarket::new()));
    market_1.borrow_mut().add_good(10.0, "Potatoes".into());

    for actor in &actors {
        workplaces[0].borrow_mut().add_worker(actor.clone());
    }

    // Initialize CSV writer
    let mut csv_writer = functions::initialize_csv_writer("simulation_data.csv").expect("Failed to create CSV writer");

    // Initialize SimulationApp

    // Simulation Step
    let mut i = 0;
    while i < n {
        info!("======= START DAY {} =======", i);

        for workplace in &workplaces {
            workplace.borrow_mut().produce();
            workplace.borrow_mut().sell_goods(market_1.clone());
            workplace.borrow_mut().profit(market_1.clone());
            workplace.borrow_mut().pay_workers();
        }
        for actor in &actors {
            actor.borrow_mut().buy_needs(market_1.clone());
            actor.borrow_mut().population_growth();
        }

        market_1.borrow_mut().update_good_price();

        // Write data to CSV and get data for plotting
        let (day, total_goods_produced, total_actors_money, technology, total_population, total_q_bought, total_q_sold) = functions::write_simulation_data(
            &mut csv_writer,
            i,
            &workplaces,
            &actors,
            &market_1,
        ).expect("Error");


        i += 1;
        workplaces[0].borrow_mut().technology = rand::thread_rng().gen_range(0.8..1.2);
        market_1.borrow_mut().new_day();

        info!("======= END DAY {} =======", i);
    }

    // Run the UI
    let native_options = eframe::NativeOptions::default();
}