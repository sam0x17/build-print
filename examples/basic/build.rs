use build_print::{println, *};

fn main() {
    println!();
    println!("testing println!:");
    println!("hello world!");
    println!("hello {}", 33);
    println!("hello {}, {}, {}", 1, 2, 3);
    println!();

    println!("testing info!:");
    info!("hello world!");
    info!("hello {}", 33);
    info!("hello {}, {}, {}", 1, 2, 3);
}
