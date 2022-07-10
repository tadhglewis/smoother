use clap::Parser;
use core::panic;
use rand::seq::SliceRandom;
use std::ops::Add;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::vec;
use tokio::task;

struct Store {
    name: String,
    blenders: Vec<Blender>,
    queue: Queue,
    // queue: Vec<Customer>,
}

#[derive(Clone, Copy)]
struct Blender {
    speed: u32,
    state: (Instant, Duration),
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

#[tokio::main]
async fn main() {
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
        store.simulate().await
    }
}

impl Store {
    pub fn new() -> Store {
        Store {
            name: "Boost Juice (Victoria Gardens".to_string(),
            blenders: vec![{
                Blender {
                    speed: 1200,
                    state: (Instant::now(), Duration::new(0, 0)),
                }
            }],
            queue: Queue { orders: vec![] },
        }
    }

    async fn simulate(&mut self) {
        self.process_orders().await;
    }

    async fn process_orders(&mut self) {
        for order in &self.queue.orders {
            let available_blender = self.get_available_blender();

            match available_blender {
                Some(mut blender) => {
                    println!("Processing order for {}", order.name);

                    blender.blend(&order.ingredients).await;

                    println!("It took {:?} to process this order", blender.state.1);
                }
                None => {}
            }
        }
    }

    fn get_available_blender(&self) -> Option<Blender> {
        let free_index = self.blenders.iter().position(|x| x.is_active() == false);

        match free_index {
            Some(index) => Some(self.blenders[index]),
            None => None,
        }
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
    async fn blend(&mut self, objects: &Vec<Object>) {
        let mut time_to_blend = Duration::new(0, 0);

        let started_at = Instant::now();

        for object in objects {
            time_to_blend = time_to_blend.add(Duration::from_secs(
                (object.blend_ms_per_w / self.speed).into(),
            ));
        }
        self.state = (started_at, time_to_blend);

        sleep(time_to_blend);
    }

    fn is_active(&self) -> bool {
        let is_state_unset = self.state.1.is_zero();
        let has_blend_time_lapsed = self.state.0.elapsed().ge(&self.state.1);

        if is_state_unset || has_blend_time_lapsed {
            false
        } else {
            true
        }
    }
}
