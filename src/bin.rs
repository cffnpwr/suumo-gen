use clap::Parser;
use suumo_gen::SuumoState;

#[derive(Debug, Parser)]
#[command(name = "suumo_gen", version, about, author)]
struct Args {
    #[arg(short, long)]
    limit: Option<usize>,

    #[cfg(feature = "multi-thread")]
    #[arg(short, long)]
    therads: Option<usize>,
}

fn main() {
    let args = Args::parse();

    gen(args);
}

#[cfg(not(feature = "multi-thread"))]
fn gen(args: Args) {
    main_loop(args.limit);
}

#[cfg(feature = "multi-thread")]
fn gen(args: Args) {
    use std::{sync::mpsc, thread};
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let threads = if let Some(threads) = args.therads {
        threads
    } else {
        sys.cpus().len()
    };
    let (tx, rx) = mpsc::channel();
    for _ in 0..threads {
        let tx = tx.clone();
        thread::spawn(move || {
            main_loop(args.limit);
            let _ = tx.send(());
        });
    }

    let _ = rx.recv();
}

fn main_loop(limit: Option<usize>) {
    let mut suumo_state = SuumoState::new();

    if let Some(limit) = limit {
        println!("{}", suumo_state.collect_to_string_nth(limit));
    } else {
        while let Some((_, element)) = suumo_state.next() {
            print!("{}", element);
        }
    }
}
