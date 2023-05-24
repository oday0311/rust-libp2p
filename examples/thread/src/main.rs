use std::thread;

fn main(){
    println!("Hello, world!");

    let mut handles = Vec::new();
    let forkSize = 40;

    for i in 0..forkSize {
        let mut str = String::from("Rust");
        let handle = thread::spawn(move || {
            // 线程中需要执行的代码
            println!("Hello from thread {}", i);
            println!("{}", str);
            let index = i;
            str.push_str(" gogo");
            println!("{}", str);
        });

        handles.push(handle);
    }

    // 等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }
}
