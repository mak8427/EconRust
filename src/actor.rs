use std::collections::HashMap;
use crate::market::Market;

pub struct item {
    
}
pub struct Actor {
    money: f32,
    name: String,
    needs: HashMap<String, f32>,
    population:i32,
    
    
    
}

impl Actor {
    pub fn new(money: f32, name: String, population: i32) -> Actor {
        
        //needs contains the amount of goods needed per person
        let mut needs = HashMap::new();
        needs.insert("Potatos".into(), 1.0);
        
        
        Actor { money,name, needs, population }
    }

    
    
    
    pub fn buy(&mut self, amount: f32) {
        self.money -= amount;
    }
    
    pub fn money_val(&self) -> f32 {
        return self.money;
    }
    
    //Needs calculate the needs of the population and returns a hashmap with the goods needed
    fn needs_calc(&self) -> HashMap<String, f32> {
        let mut goods_needed = self.needs.clone();
        for (key, value) in &mut goods_needed {
            if key == "Potatos" {
                *value *= self.population as f32;
            }
        }
        goods_needed
    }
    
    
    pub fn  buy_needs(&self,market: &Market){
        let goods_needed = self.needs_calc();
        
        //Iterate over the goods needed and buy them
        for (key, value) in &goods_needed { 
            let good = market.get_good(key);
            
            if good.is_none() {
                println!("Good not found: {}", key);
                break;
            }
            
            let price = good.unwrap().price;
            let amount = value * price;
            println!("Buying {} for {} each", key, price);
            println!("Amount: {}", amount);
        }
        println!("Goods needed: {:?}", goods_needed);
    }
    
}