
use log::{info};
pub struct Good {
    pub(crate) price: f32,
    name: String,
    q_sold: f32,
    q_bought: f32,
    q_demanded: Vec<f32>,
    q_supplied: Vec<f32>,
}
pub struct Market {
    goods: Vec<Good>,
}
impl Market {
    pub fn new() -> Market {
        Market { goods: Vec::new() }
    }
    pub fn add_good(&mut self, price: f32, name: String) {
        self.goods.push(Good { price, name , q_sold: 0.0, q_bought: 0.0, q_demanded: Vec::new(), q_supplied: Vec::new() });
    }
    pub fn get_good(&self, key: &String) -> Option<&Good>{
        for good in self.goods.iter() {
            if good.name == *key {
                return Some(good);
            }
        }
        None
    }
    
    pub fn new_day(&mut self) {
        for good in self.goods.iter_mut() {
            good.q_demanded.push(good.q_sold );
            good.q_supplied.push(good.q_bought );
            good.q_sold = 0.0;
            good.q_bought  = 0.0;
        }
    }
    
    pub fn increase_q_sold(&mut self, name: String, amount: f32) {
        for good in self.goods.iter_mut() {
            if good.name == name {
                good.q_sold += amount;
                info!("Sold {} of {}, with a total of {}", amount, name, good.q_sold);
            }
        }
    }
    pub fn increase_q_bought(&mut self, name: String, amount: f32) {
        for good in self.goods.iter_mut() {
            if good.name == name {
                good.q_bought += amount;
                info!("Bought {} of {}, with a total of {}", amount, name, good.q_bought);
            }
        }
    }
    
    pub fn update_good_price(&mut self) {
        for good in self.goods.iter_mut() {
            good.price =  good.price * (good.q_bought / good.q_sold).sqrt();
            info!("The price of {} is now {}, Q_bought: {} , Q_Sold: {}", good.name, good.price, good.q_bought , good.q_sold);
        }
    }
    
}