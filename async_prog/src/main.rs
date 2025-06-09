use std::time::Instant;
use tokio::task;

fn heavy_cpu_task(id: usize) {
    let mut count = 0;
    for _ in 0..5000 {
        count += 1;
    }

    println!("Task {id} done {count}");
}

#[tokio::main]
async fn main() {
    let now = Instant::now();

    let logical = num_cpus::get();
    let physical = num_cpus::get_physical();
    println!("CPU {logical}T / {physical}C");

    let mut handles = vec![];
    for i in 0..logical {
        handles.push(task::spawn_blocking(move || {
            heavy_cpu_task(i);
        }));
    }

    // Wait for all tasks to complete
    for h in handles {
        let _ = h.await;
    }

    println!("All tasks completed in {:.2?}", now.elapsed());
}
