mod MasterNode;
mod SlaveNode;

use std::thread;
use async_std::prelude::FutureExt;
use async_std::task;



//async functions
fn main(){
    //let rt = Runtime::new().unwrap();
    let mut localtasks = Vec::new();
    let masterSize = 2;
    let slaveSize = 1;


    for i in 0..masterSize {

        let task = task::spawn(async move  {
            let result = MasterNode::startNode().await;
        });

        localtasks.push(task);
    }


    for i in  0..slaveSize {
        let task = task::spawn(async move  {
            let result = SlaveNode::startNode().await;
        });

        localtasks.push(task);

    }


    // 等待所有线程执行完毕
    for t in localtasks {
        //handle.join().unwrap();
        task::block_on(t);
    }


    task::block_on(async {
        println!("Hello, world!  task ========");
    });




}


//we can use the following to fork sync functions. but no async functions
fn mainSyncSpawn(){
    println!("Hello, world!");

    let mut handles = Vec::new();
    let forkSize = 3;

    for i in 0..forkSize {
        let mut str = String::from("Rust");
        let handle = thread::spawn(move || {
            // 线程中需要执行的代码
            println!("Hello from thread {}", i);
            println!("{}", str);
            let index = i;
            str.push_str(" gogo");
            println!("{}", str);


            loop {
                println!("Hello from thread {}", index);
                thread::sleep_ms(1000);
            }
        });

        handles.push(handle);
    }

    // 等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }




}
