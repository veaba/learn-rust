#![allow(unused)]
fn main() {
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(10);
    println!("{:?}",ten_millis);
    thread::sleep(ten_millis);
    thread::sleep(ten_millis*100);
}



//
//use std::sync::Arc;
//use std::thread;
//use std::time::Duration;
//
//struct JobStatus {
//    jobs_completed: i32,
//}
//
//fn main() {
//    let status = Arc::new(JobStatus { jobs_completed: 0 });
//    let status_shared = status.clone();
//    thread::spawn(move || {
//        for _ in 0..10 {
//            thread::sleep(Duration::from_millis(250));
//
//            println!("{}",status_shared.jobs_completed);
////            status_shared.jobs_completed += 1;
//        }
//    });
//    while status.jobs_completed < 10 {
//        println!("waiting... ");
//        thread::sleep(Duration::from_millis(500));
//    }
//}
