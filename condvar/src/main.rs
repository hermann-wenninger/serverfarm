use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Gemeinsamer Speicher: (Vector, gezählte Primzahlen)
    let pair = Arc::new((Mutex::new(Vec::new()), Condvar::new()));

    let pair_producer = Arc::clone(&pair);

    // PRODUCER: berechnet Primzahlen und pusht sie in den Vector
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*pair_producer;

        let mut count = 0;
        let mut n = 2;

        loop {
            if is_prime(n) {
                let mut data = lock.lock().unwrap();
                data.push(n);
                count += 1;

                // Wenn 1000 Primzahlen gefunden -> Wecke den anderen Thread
                if count == 1000000 {
                    println!("Producer: 1000 Primzahlen berechnet, wecke Consumer!");
                    cvar.notify_one();
                }
            }
            n += 1;

            if count >= 1000000 {
                break;
            }
        }
    });

    let pair_consumer = Arc::clone(&pair);

    // CONSUMER: wartet, bis 1000 Primzahlen vorhanden sind
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair_consumer;

        let mut data = lock.lock().unwrap();

        // Warte, bis Bedingung erfüllt (mind. 1000 Primzahlen im Vector)
        while data.len() < 1000000 {
            println!("Consumer: Warte auf genügend Primzahlen...");
            data = cvar.wait(data).unwrap();
        }

        println!("Consumer: Habe {} Primzahlen im Buffer!", data.len());
        println!("Erste 10 Primzahlen: {:?}", &data[999995..1000000]);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}