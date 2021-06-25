use std::sync::mpsc;
use std::thread;

fn main() {
    // tx: 送信インスタンス rx: 受信インスタンス
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello, world!");
    let _ = handle.join();
}
