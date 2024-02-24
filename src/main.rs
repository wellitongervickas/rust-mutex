use std::sync::{Arc, Mutex};
use std::thread;
// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num: MutexGuard<'_, i32> = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {:?}", m);
// }

// fn main() {
//     let var_name = Rc::new(Mutex::new(0));
//     let counter = var_name;
//     let mut handles: Vec<thread::JoinHandle<()>> = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);

//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;

//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let mut num: std::sync::MutexGuard<'_, i32> = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
