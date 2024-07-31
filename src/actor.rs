use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::{info};
use crate::market::Market;

pub struct Item {
    
}
pub struct Actor {
    money: f32,
    name: String,
    needs: HashMap<String, i32>,
    population:i32,
    
    
    
    
}

impl Actor {
    pub fn new(money: f32, name: String, population: i32) -> Actor {
        
        //needs contains the amount of goods needed per person
        let mut needs = HashMap::new();
        needs.insert("Potatoes".into(), 1);
        
        
        Actor { money,name, needs, population }
    }

    
    
    
    pub fn buy(&mut self, amount: f32) {
        self.money -= amount;
    }
    pub fn get_paid(&mut self, amount: f32) {  self.money += amount; }
    pub fn money_val(&self) -> f32 {
        return self.money;
    }
    
    
    pub fn population_val(&self) -> i32 { return self.population; }
    pub fn increase_population(&mut self, amount: i32) {
        self.population += amount;
    }
    
    
    
    
    
    
    
    pub fn decrease_population(&mut self, amount: i32) { self.population -= amount; }
    
    
    
    
    
    
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
    
    
    pub fn  buy_needs(&mut self, market: Rc<RefCell<Market>>){
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
            let amount = (*value as f32)  * price;
           
            
            if amount > self.money {
                info!("Not enough money to buy {} for {} each", key, price);
                info!("Gold Needed {}", amount - self.money);
                continue;
            }
            if amount < self.money {
                info!("Buying {} for {} each", key, price);
                self.buy(amount);
                let value_ : i32 = *value;
                market_borrow.increase_q_bought(key.clone(), value_);
            }
            info!("Amount: {}", amount);
        }
        info!("Goods needed: {:?}", goods_needed);
    }
    
}