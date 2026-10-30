mod memory;
mod loader;

fn main() {
    println!("Atlas Memory Loader v0.1");

    loader::load_memories();

    println!("Atlas startup successful.");
}
