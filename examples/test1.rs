/////////////////////////////////////////////////////////////
// thread_result::test1.rs - basic ThreadResult test       //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 Apr 2021  //
/////////////////////////////////////////////////////////////

use thread_result::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn test() {
    let thrd_rslt = Arc::new(ThreadResult::<i32>::new());
    let thrd_rslt1 = Arc::clone(&thrd_rslt);
    let thrd_rslt2 = Arc::clone(&thrd_rslt);

    let cls = move || {
        let dur = Duration::from_secs(10u64);
        thread::sleep(dur);
        thrd_rslt2.set(42);
    };
    let handle = thread::spawn(cls);

    let dur = Duration::from_secs(1);
    loop {
        if !thrd_rslt1.ready() {
            print!("\n  main waiting");
            thread::sleep(dur);
        }
        else {
            print!("\n  thread result is {}", thrd_rslt.get());
            break;
        }
    }
    let _ = handle.join();
}
fn main() {
    test();
}