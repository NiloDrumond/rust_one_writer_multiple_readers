use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};
use threadpool::ThreadPool;

fn consumer(index: u32, arc_value: Arc<AtomicU8>, arc_consumers: Arc<AtomicU8>) {
    // Equivalente a consumir()
    let state = arc_value.clone();
    loop {
        sleep(Duration::from_millis(10));
        let loaded = state.load(Ordering::SeqCst);
        if loaded != 0 {
            println!("C {:?}: {:?}", index, loaded);
            // Subtrai 1 no contador de consumidores
            arc_consumers.fetch_sub(1, Ordering::SeqCst);
            if arc_consumers.load(Ordering::SeqCst) == 0 {
                // Caso seja este seja o ultimo consumidor, limpa o value produzido pelo produtor
                arc_value.fetch_min(0, Ordering::SeqCst);
            }
            break;
        }
    }
}

fn producer(arc_state: Arc<AtomicU8>) {
    // Equivalente a função produzir(x)
    arc_state.fetch_add(rand::random::<u8>(), Ordering::SeqCst);
}

fn main() {
    let pool = ThreadPool::new(100);
    let arc_value = Arc::new(AtomicU8::new(0));
    let arc_consumers = Arc::new(AtomicU8::new(0));
    let mut index = 0;

    loop {
        sleep(Duration::from_millis(100));
        let arc_value = arc_value.clone();
        let is_consumer = rand::random::<bool>();
        if is_consumer {
            // Cria um consumidor
            let arc_consumers = arc_consumers.clone();
            index += 1;
            arc_consumers.fetch_add(1, Ordering::SeqCst);
            pool.execute(move || {
                consumer(index, arc_value, arc_consumers);
            })
        } else if arc_value.load(Ordering::SeqCst) == 0 {
            // Cria um produtor
            thread::spawn(move || {
                producer(arc_value);
            });
        }
    }
}
