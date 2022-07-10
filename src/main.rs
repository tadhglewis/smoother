use clap::Parser;
use core::panic;
use rand::seq::SliceRandom;
use std::ops::{Add, Sub};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};
use std::vec;

struct Store {
    name: String,
    blenders: Vec<Blender>,
    queue: Queue,
    // open: bool,
    // queue: Vec<Customer>,
}

#[derive(Clone, Copy)]
struct Blender {
    speed: u32,
    state: (Instant, Duration),
}

#[derive(Clone)]
struct Queue {
    orders: Vec<Order>,
}

#[derive(Clone)]
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

impl std::fmt::Display for Blender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r"
 __=__
|     |
|     |
 \ _ /
 [---]
[  O  ]
[_____]
",
        )
        .expect("Failed to render blender");

        if self.is_active() {
            write!(
                f,
                "{}/{}",
                self.state.0.elapsed().as_secs(),
                self.state.1.as_secs()
            )
        } else {
            write!(f, "Machine available")
        }
    }
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

    store.queue.add(Order {
        name: "Jahred".to_string(),
        ingredients: [
            Object {
                name: "Ice".to_string(),
                blend_ms_per_w: 9000,
            },
            Object {
                name: "Rocks".to_string(),
                blend_ms_per_w: 90000,
            },
        ]
        .to_vec(),
    });

    store.queue.add(Order {
        name: "Tadhg".to_string(),
        ingredients: [Object {
            name: "Ice".to_string(),
            blend_ms_per_w: 9000,
        }]
        .to_vec(),
    });

    loop {
        clearscreen::clear().unwrap();
        println!("----------");
        for blender in &store.blenders {
            println!("{}", blender);
        }
        println!("----------");
        println!("Queue size: {}", store.queue.clone().size());
        store.tick();
    }
}

impl Store {
    pub fn new() -> Store {
        Store {
            name: "Boost Juice (Victoria Gardens".to_string(),
            blenders: vec![
                Blender {
                    speed: 1200,
                    state: (Instant::now(), Duration::new(0, 0)),
                },
                Blender {
                    speed: 2000,
                    state: (Instant::now(), Duration::new(0, 0)),
                },
            ],
            queue: Queue { orders: vec![] },
        }
    }

    fn tick(&mut self) {
        // Check for orders and blend the if a blender is available
        self.process_orders();
        // CLeanup blenders if the timer has finished
        self.clean_blenders();
    }

    fn process_orders(&mut self) {
        if self.queue.orders.len() != 0 {
            let order = self.queue.orders[0].clone();

            let available_blender = self.get_available_blender();

            match available_blender {
                Some(blender) => {
                    blender.start_blend(&order.ingredients);
                    self.queue.remove()
                }
                None => (),
            }
        }
    }

    fn clean_blenders(&mut self) {
        for blender in self.blenders.iter_mut() {
            if !blender.is_active() && !blender.state.1.is_zero() {
                blender.clean();
            }
        }
    }

    fn get_available_blender(&mut self) -> Option<&mut Blender> {
        let free_index = self.blenders.iter().position(|x| !x.is_active());

        match free_index {
            Some(index) => Some(&mut self.blenders[index]),
            None => None,
        }
    }
}

impl Queue {
    pub fn size(self) -> usize {
        self.orders.len()
    }

    pub fn add(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn remove(&mut self) {
        self.orders.swap_remove(0);
    }
}

impl Blender {
    fn start_blend(&mut self, objects: &Vec<Object>) {
        let mut time_to_blend = Duration::new(0, 0);

        let started_at = Instant::now();

        for object in objects {
            time_to_blend = time_to_blend.add(Duration::from_secs(
                (object.blend_ms_per_w / self.speed).into(),
            ));
        }

        self.state = (started_at, time_to_blend);
    }

    fn clean(&mut self) {
        self.state = (Instant::now(), Duration::new(0, 0))
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
