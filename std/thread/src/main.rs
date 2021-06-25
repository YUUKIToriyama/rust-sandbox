use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // lock()で排他処理を可能に
            // Mutex#lock()は複数のスレッドが同時にアクセスしないようにするメソッド
            let mut data = data_ref.lock().unwrap();
            data[x] = data[x] + 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
