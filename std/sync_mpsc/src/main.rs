use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // main()から各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        snd_channels.push(snd_tx);
        // 各スレッドからmain()へのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            // main()からの受信をdataに紐付ける
            let mut data = snd_rx.recv().unwrap();
            data = data + 1;
            // main()にdataを送り返す
            let _ = rcv_tx.send(data);
        }));
    }

    // 各スレッドにdataの値を送信する
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }
    // 各スレッドからの結果をdataに格納
    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
