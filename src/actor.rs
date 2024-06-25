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
        let mut needs = HashMap::new();
        needs.insert("Potatos".into(), 0.0);
        Actor { money,name, needs, population }
    }

    
    
    
    pub fn buy(&mut self, amount: f32) {
        self.money -= amount;
    }
    
    pub fn money_val(&self) -> f32 {
        return self.money;
    }
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
        for (key, value) in &goods_needed {
            let good = market.get_good(key);
            let price = good.price;
            let amount = value * price;
            println!("Buying {} for {} each", key, price);
            println!("Amount: {}", amount);
        }
        println!("Goods needed: {:?}", goods_needed);
    }
    
}