use wgpu_tutorial::run;

fn main() {
    println!("Application starting...");
    match run() {
        Ok(_) => {
            println!("Application exited successfully.");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("Application finished.");
}