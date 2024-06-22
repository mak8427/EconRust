use rand::Rng;

fn main() {
    println!("Hello, world!");
            
            struct Actor {
                money : i32,
                name: String,
            }

            impl Actor {
                fn new(money: i32, name: String) -> Actor {
                    Actor { money,name }
                }

                fn nome(&self) {
                    println!("{}", self.name);
                }

                fn money_val(&self) -> i32 {
                    return self.money;
                    
                }
            }

            let my_class = Actor::new(rand::thread_rng().gen_range(0..50), "Test".into());
            my_class.nome();
            println!("{}", my_class.money_val());
        }