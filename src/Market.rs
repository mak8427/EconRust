pub struct Good {
    price: f32,
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
    pub fn get_good(&self, index: usize) -> &Good {
        &self.goods[index]
    }
    
}