use std::{sync::Arc, thread};

trait Engine: Send + Sync {
    fn start(&self);
}

struct Car;
struct Bike;

impl Engine for Car {
    fn start(&self) {
        println!("Car engine started");
    }
}

impl Engine for Bike {
    fn start(&self) {
        println!("Bike engine started");
    }
}

fn main() {
    let car: Arc<dyn Engine> = Arc::new(Car);

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let shared_car = Arc::clone(&car);
            thread::spawn(move || {
                shared_car.start();
            })
        })
        .collect();

    for h in handles {
        h.join().expect("Couldn't join a thread");
    }

    /*

    let vehicles: Vec<Box<dyn Engine>> = vec![Box::new(Car), Box::new(Bike)];

    vehicles.into_iter().for_each(|engine| {
        thread::spawn(move || {
            engine.start();
        })
        .join()
        .expect("Couldn't join a thread");
    })

    */

    /*

    // Using i32 for x
    // Primitive types like i32 are Copy, Send, and Sync by default, meaning they can be safely shared between threads without additional synchronization mechanisms.
    // You do not need to wrap i32 in an Arc to share it between threads.
    let x = 4;

    let handles: Vec<_> = (0..3)
        .map(|_| {
            thread::spawn(move || {
                println!("Hello, world! x is {}", x);
            })
        })
        .collect();

    for h in handles {
        h.join().expect("Couldn't join a thread");
    }

    // Using String for x
    // String is a heap-allocated, growable string type in Rust.
    // String is not Copy, meaning when you assign or pass a String to another variable or function, ownership is transferred, and the original variable is no longer valid.
    // String is also not Sync by default, meaning it cannot be safely shared between threads without additional synchronization mechanisms.
    // To safely share a String between multiple threads, you need to wrap it in an Arc (Atomic Reference Counted) pointer.
    // Arc::clone is used to create a new reference to the same String data, allowing multiple threads to access it concurrently.
    let x = Arc::new(String::from("Hello, world!"));

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let shared_x = Arc::clone(&x);
            thread::spawn(move || {
                println!("Hello, world! x is {}", shared_x);
            })
        })
        .collect();

    for h in handles {
        h.join().expect("Couldn't join a thread");
    }

    */
}
