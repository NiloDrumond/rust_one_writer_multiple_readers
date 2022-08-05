use std::{sync::{Arc, atomic::{Ordering, AtomicU8}}, thread::sleep, time::Duration};
use threadpool::ThreadPool;


fn consumer(index: u32, arc_value: Arc<AtomicU8>, arc_consumers: Arc<AtomicU8>) {
    let state = arc_value.clone();
    loop {
        sleep(Duration::from_millis(100));
        if state.load(Ordering::SeqCst) != 0 {
            println!("C {:?}: {:?}",index, state.load(Ordering::SeqCst));
            arc_consumers.fetch_sub(1, Ordering::SeqCst);
            if arc_consumers.load(Ordering::SeqCst) == 0 {
                arc_value.fetch_min(0, Ordering::SeqCst);
            }
            break;
        }
    }
}

fn producer(arc_state:  Arc<AtomicU8>) {
    arc_state.fetch_add(rand::random::<u8>(), Ordering::SeqCst); 
}

fn main() {
    let pool = ThreadPool::new(100);
    let arc_value = Arc::new(AtomicU8::new(0));
    let arc_consumers = Arc::new(AtomicU8::new(0));
    let mut index = 0;


    loop {
        sleep(Duration::from_millis(500));
        let arc_value = arc_value.clone();
        let is_consumer = rand::random::<bool>();
        if is_consumer {
            let arc_consumers = arc_consumers.clone();
            index += 1;
            arc_consumers.fetch_add(1, Ordering::SeqCst);
            pool.execute(move || {
                consumer(index, arc_value, arc_consumers);
            })
        } else if arc_value.load(Ordering::SeqCst) == 0 {
            producer(arc_value);
        }
    }
}
