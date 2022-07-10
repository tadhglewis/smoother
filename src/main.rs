use core::panic;
use rand::seq::SliceRandom;
use std::ops::Add;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use clap::Parser;

#[derive(Clone)]
struct Object {
    // Blend time in MS per blender watt power
    blend_time_per_w: u32,
    name: String,
}

struct Order {
    name: String,
    ingredients: Vec<Object>,
}

struct Queue {
    orders: Vec<Order>,
}

struct Store {
    blenders: Vec<Blender>,
    // open: bool,
    queue: Queue,
}

#[derive(Copy, Clone, Debug)]
struct Blender {
    // Blender watts
    speed: u32,
    active: bool,
}

#[derive(Debug, Parser)]
struct Cli {
    command: String,
}

fn main() {
    let args = Cli::parse();

    let mut store = Store::new();

    // let names = vec!["Tadhg", "Jahred", "Ben"];
    // let name = names.choose(&mut rand::thread_rng());

    store.queue.add(Order {
        name: "Jahred".to_string(),
        ingredients: [
            Object {
                name: "Ice".to_string(),
                blend_time_per_w: 9000,
            },
            Object {
                name: "Banana".to_string(),
                blend_time_per_w: 1000,
            },
        ]
        .to_vec(),
    });

    loop {
        store.process_orders();
    }
}

impl Store {
    pub fn new() -> Store {
        Store {
            blenders: vec![Blender {
                speed: 10000,
                active: false,
            }],
            queue: Queue { orders: vec![] },
            // open: false,
        }
    }

    // pub fn open(&mut self) {
    //     self.open = true;
    // }

    // pub fn close(&mut self) {
    //     self.open = false;
    // }

    fn get_inactive_blender(&self) -> Blender {
        let free_index = self
            .blenders
            .iter()
            .position(|x| x.active == false)
            .unwrap();

        self.blenders[free_index]
    }

    fn process_orders(&mut self) {
        for order in &self.queue.orders {
            let free_blender = self.get_inactive_blender();

            let time = free_blender.blend_objects(&order.ingredients);

            println!(
                "It took {:?} to blend order: {}",
                time, self.queue.orders[0].name
            );
        }
    }
}

impl Queue {
    pub fn size(&self) -> usize {
        self.orders.len()
    }

    pub fn add(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn remove(&mut self, i: usize) {
        self.orders.remove(i);
    }
}

impl Blender {
    fn blend_objects(&self, objects: &Vec<Object>) -> Duration {
        let mut time_to_blend = Duration::new(0, 0);

        for object in objects {
            time_to_blend = time_to_blend.add(Duration::from_secs(
                (object.blend_time_per_w / self.speed).into(),
            ));
        }

        sleep(time_to_blend);

        time_to_blend
    }
}
