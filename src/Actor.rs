
pub struct Actor {
    money: f32,
    name: String,
}

impl Actor {
    pub fn new(money: f32, name: String) -> Actor {
        Actor { money,name }
    }
    pub fn buy(&mut self, amount: f32) {
        self.money -= amount;
    }
    
    pub fn money_val(&self) -> f32 {
        return self.money;

    }
}