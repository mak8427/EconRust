pub struct Good {
    price: f32,
    name: String,
    Q_sold: f32,
    Q_bought: f32,
    Q_demanded: Vec<f32>,
    Q_supplied: Vec<f32>,
}
pub struct Market {
    goods: Vec<Good>,
}
impl Market {
    pub fn new() -> Market {
        Market { goods: Vec::new() }
    }
    pub fn add_good(&mut self, price: f32, name: String) {
        self.goods.push(Good { price, name , Q_sold: 0.0, Q_bought: 0.0, Q_demanded: Vec::new(), Q_supplied: Vec::new() });
    }
    pub fn get_good(&self, index: usize) -> &Good {
        &self.goods[index]
    }
}