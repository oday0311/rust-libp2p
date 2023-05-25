mod MasterNode;
mod SlaveNode;

use std::error::Error;
use std::thread;
use async_std::prelude::FutureExt;
use futures::{prelude::*, select};
use async_std::{io, task};


const masterSize: i32 = 1;
const slaveSize: i32 = 2;

//async functions
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let rt = Runtime::new().unwrap();

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Kick it off.
    loop {
        println!("current msg is 1");
        select! {
            line = stdin.select_next_some() => handle_input_line(line.expect("read error")),

        }
    }



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


fn handle_input_line( line: String) {


    let mut args = line.split(' ');
    match args.next() {
        Some("master") => {
            let mut localtasks = Vec::new();

            for i in 0..masterSize {

                let task = task::spawn(async move   {
                    let result = MasterNode::startNode().await;
                });

                localtasks.push(task);
            }

            // 等待所有线程执行完毕
            for t in localtasks {
                //handle.join().unwrap();
                task::block_on(t);
            }


        }
        Some("slave") => {
            let mut localtasks = Vec::new();

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

        }

        _ => {
            println!("Invalid input: please add type master or slave");
        }
    }
}
