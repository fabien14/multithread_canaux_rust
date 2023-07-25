use std::thread;
use std::sync::mpsc::{ channel, Sender };
use std::time::Duration;

fn main() {
    
    let (tx, rx) = channel();
    let tx_clone = Sender::clone(&tx);
    
    let thread_1 = thread::spawn(move || {
        let interval = Duration::from_secs(1);
        let id = thread::current().id();

        for i in 1..10 {
            let msg = format!("thread {:?} : Msg {}", id, i);
            tx.send(msg).unwrap();
            thread::sleep(interval);
        }
    });

    let thread_2 = thread::spawn(move || {

        for msg_received in rx {
            println!("{}", msg_received);
        } 
    });

    let thread_3 = thread::spawn(move || {
        let interval = Duration::from_secs(2);
        let id = thread::current().id();

        for i in 1..10 {
            let msg = format!("thread {:?} : Msg {}", id, i);
            tx_clone.send(msg).unwrap();
            thread::sleep(interval);
        }
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
}
