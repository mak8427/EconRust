use rand::Rng;

fn main() {
    println!("Hello, world!");
            
            struct Actor {
                money : f32,
                name: String,
            }

            impl Actor {
                fn new(money: f32, name: String) -> Actor {
                    Actor { money,name }
                }
                fn buy(&mut self, amount: f32) {
                    self.money -= amount;
                }

                fn nome(&self) {
                    println!("{}", self.name);
                }
                fn money_val(&self) -> f32 {
                    return self.money;
                    
                }
            }

            let mut my_class = Actor::new(rand::thread_rng().gen_range(0.0..50.0), "Test".into());

            my_class.buy(-10.0);
            println!("{}", my_class.money_val());
        }