use std::collections::HashMap;

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
    pub fn  needs(&self) -> f32 {
        return 0.0;
    }
    
}