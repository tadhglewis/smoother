use clap::Parser;
use core::panic;
use rand::seq::SliceRandom;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

struct Store {
    name: String,
    blenders: Vec<Blender>,
    queue: Queue,
    // queue: Vec<Customer>,
}

#[derive(Clone, Copy)]
struct Blender {
    speed: u32,
    active: bool,
}

struct Queue {
    orders: Vec<Order>,
}

struct Order {
    name: String,
    ingredients: Vec<Object>,
}

#[derive(Clone)]
struct Object {
    blend_ms_per_w: u32,
    name: String,
}

#[derive(Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();

    let mut store = Store::new();

    store.queue.add(Order {
        name: "Jahred".to_string(),
        ingredients: [
            Object {
                name: "Ice".to_string(),
                blend_ms_per_w: 9000,
            },
            Object {
                name: "Banana".to_string(),
                blend_ms_per_w: 1000,
            },
        ]
        .to_vec(),
    });

    // simulation loop
    loop {
        store.simulate()
    }
}

impl Store {
    pub fn new() -> Store {
        Store {
            name: "Boost Juice (Victoria Gardens".to_string(),
            blenders: vec![{
                Blender {
                    speed: 1200,
                    active: false,
                }
            }],
            queue: Queue { orders: vec![] },
        }
    }

    fn simulate(&mut self) {
        self.process_orders();
    }

    fn process_orders(&mut self) {
        for order in &self.queue.orders {
            println!("Processing order for {}", order.name);

            let available_blender = self.get_available_blender();

            let time = available_blender.blend(&order.ingredients);

            println!("It took {:?} to process this order", time);
        }
    }

    fn get_available_blender(&self) -> Blender {
        let free_index = self
            .blenders
            .iter()
            .position(|x| x.active == false)
            .unwrap();

        self.blenders[free_index]
    }
}

impl Queue {
    pub fn size(self) -> usize {
        self.orders.len()
    }

    pub fn add(&mut self, order: Order) {
        self.orders.push(order)
    }
}

impl Blender {
    fn blend(&self, objects: &Vec<Object>) -> Duration {
        let mut time_to_blend = Duration::new(0, 0);

        for object in objects {
            time_to_blend = time_to_blend.add(Duration::from_secs(
                (object.blend_ms_per_w / self.speed).into(),
            ));
        }

        sleep(time_to_blend);

        time_to_blend
    }
}
