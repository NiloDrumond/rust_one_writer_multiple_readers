use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};
use threadpool::ThreadPool;

fn consumer(index: u32, arc_value: Arc<AtomicU8>, arc_consumers: Arc<AtomicU8>) {
    let state = arc_value.clone();
    loop {
        sleep(Duration::from_millis(100));
        if state.load(Ordering::Relaxed) != 0 {
            println!("C {:?}: {:?}", index, state.load(Ordering::Relaxed));
            arc_consumers.fetch_sub(1, Ordering::Relaxed);
            if arc_consumers.load(Ordering::Relaxed) == 0 {
                arc_value.fetch_min(0, Ordering::Relaxed);
            }
            break;
        }
    }
}

fn producer(arc_state: Arc<AtomicU8>) {
    arc_state.fetch_add(rand::random::<u8>(), Ordering::Relaxed);
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
            arc_consumers.fetch_add(1, Ordering::Relaxed);
            pool.execute(move || {
                consumer(index, arc_value, arc_consumers);
            })
        } else if arc_value.load(Ordering::Relaxed) == 0 {
            producer(arc_value);
        }
    }
}
