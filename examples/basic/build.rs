use build_print::{println, *};

fn main() {
    println!();
    println!("testing println!:");
    println!("hello world!");
    println!("hello {}", 33);
    println!("hello {}, {}, {}", 1, 2, 3);
    println!();

    info!("testing info!");
    info!("hello {}", 33);
    info!("hello {}, {}, {}", 1, 2, 3);
    println!();

    warn!("testing warn!");
    warn!("hello {}", 33);
    warn!("hello {}, {}, {}", 1, 2, 3);
    println!();

    error!("testing error!");
    error!("hello {}", 33);
    error!("hello {}, {}, {}", 1, 2, 3);
    println!();

    note!("testing note!");
    note!("hello {}", 33);
    note!("hello {}, {}, {}", 1, 2, 3);
    println!();
}
