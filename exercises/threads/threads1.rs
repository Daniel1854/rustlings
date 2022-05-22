// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// TODO: do this exercise again, feels like the docs has changed since then
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status = Arc::clone(&status);
        let handle =thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut job_status = status.lock().unwrap();
            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    };

    // what is this original abomination of spinlock,
    // not quite sure how one would have to model this with mutexes
    // but just gonna replace them with a fork join model
    for handle in handles {
        println!("waiting... ");
        handle.join().unwrap();
    }

    print!("Result: {}", status.lock().unwrap().jobs_completed);
}
