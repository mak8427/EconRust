use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::{info};
use crate::market::Market;

pub struct Item {}

pub struct Actor {
    money: f32,
    name: String,
    needs: HashMap<String, i32>,
    population: i32,
    primary_needs_satisfaction: HashMap<String, f32>,
    growth_rate: f32,
}

impl Actor {
    pub fn new(money: f32, name: String, population: i32, growth_rate: f32) -> Actor {

        //needs contains the amount of goods needed per person
        let mut needs = HashMap::new();
        let mut needs_satisfaction = HashMap::new();
        needs.insert("Potatoes".into(), 1);
        needs_satisfaction.insert("Potatoes".into(), 1.0);


        Actor { money, name, needs, population, primary_needs_satisfaction: needs_satisfaction, growth_rate }
    }


    pub fn buy(&mut self, amount: f32) {
        self.money -= amount;
    }
    pub fn get_paid(&mut self, amount: f32) { self.money += amount; }
    pub fn money_val(&self) -> f32 {
        return self.money;
    }


    pub fn population_val(&self) -> i32 { return self.population; }
    pub fn increase_population(&mut self, amount: i32) { self.population += amount; }
    pub fn decrease_population(&mut self, amount: i32) { self.population -= amount; }

    pub fn population_growth(&mut self){
        let avg_satisfaction: f32 = self.primary_needs_satisfaction.values().copied().sum::<f32>() / self.primary_needs_satisfaction.len() as f32;
        
        if avg_satisfaction > 0.8 {
            let pop_delta: i32 = (self.growth_rate * self.population as f32 * avg_satisfaction).round() as i32 + 1;
            self.increase_population(pop_delta);
            info!("Avg_satisfaction: {}  --> Increase in Population by {} total of  {}", avg_satisfaction, pop_delta, self.population);
        }
        else {
            let pop_delta: i32 = (self.population as f32 *(0.99 - avg_satisfaction)).round() as i32 ;
            self.decrease_population(pop_delta);
            info!("Avg_satisfaction: {}  --> Decrease in Population by {} total of  {}", avg_satisfaction, pop_delta, self.population);
            
        }


    }


    //Calculate the needs of the population and returns a hashmap with the goods needed
    fn needs_calc(&self) -> HashMap<String, i32> {
        let mut goods_needed = self.needs.clone();
        for (key, value) in &mut goods_needed {
            if key == "Potatoes" {
                *value = self.population;
            }
        }
        goods_needed
    }


    pub fn buy_needs(&mut self, market: Rc<RefCell<Market>>) {
        let goods_needed = self.needs_calc();
        let mut market_borrow = market.borrow_mut();


        //Iterate over the goods needed and buy them
        for (key, value) in &goods_needed {
            let good = market_borrow.get_good(key);

            //Check if the good exists
            if good.is_none() {
                info!("Good not found: {}", key);
                break;
            }

            let price = good.unwrap().price;
            let amount = (*value as f32) * price;


            if amount > self.money {
                info!("Not enough money to buy {} for {} each", key, price);
                info!("Gold Needed {}", amount - self.money);
                self.primary_needs_satisfaction.insert(key.clone(), self.money / amount);

                continue;
            }
            if amount < self.money {
                info!("Buying {} for {} each", key, price);
                self.buy(amount);
                let value_: i32 = *value;
                self.primary_needs_satisfaction.insert(key.clone(), 1.0);
                market_borrow.increase_q_bought(key.clone(), value_);
            }
            info!("Amount: {}", amount);
        }
        info!("Goods needed: {:?}", goods_needed);
    }
}