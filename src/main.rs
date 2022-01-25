use libtrusted;
use libuntrusted;

use clap::{App, Arg};
use serde::Serialize;
use std::error::Error;
use std::io;
use std::fs::File;
use std::time::Instant;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    work: usize,
    uninstrumented: f64,
    callgate: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("PKRU-Safe Bench")
        .author("Paul Kirth <pkirth@uci.edu>")
        .about("Measures PKRU-Safe instrumentation overheads")
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("num")
                .takes_value(true)
                .help("number of iterations to run each test"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output csv file path"),
        )
        .get_matches();

    let iowrt: Box<dyn io::Write> = if let Some(n) = matches.value_of("output") {
        Box::new(File::create(n)?)
    } else {
        Box::new(io::stdout())
    };

    let mut wtr = csv::Writer::from_writer(iowrt);

    let mut iter: usize = 100;
    if let Some(n) = matches.value_of("num") {
        iter = n.parse().unwrap();
    }
    println!("Running benchmarks for {} iterations", iter);
    let mut res = vec![];

    for i in (0..1000).step_by(50) {
        do_nothing_bench(iter, i, &mut res);
    }
    for i in (1000..10001).step_by(500) {
        do_nothing_bench(iter, i, &mut res);
    }

    for r in res {
        wtr.serialize(r)?;
    }

    wtr.flush()?;
    Ok(())
}

fn do_nothing_bench(iter: usize, ms: usize, results: &mut Vec<Record>) {
    let mut x = Box::new(5);
    let start = Instant::now();
    for _ in 0..iter {
        x = libtrusted::safe_do_nothing(x, ms);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        x = libuntrusted::safe_do_nothing(x, ms);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    results.push(Record {
        work: ms,
        uninstrumented: t,
        callgate: u,
    });
    //println!("Do nothing {}us, {}, {}", ms, t, u);
}

#[allow(unused)]
fn add_bench(iter: usize) {
    let start = Instant::now();
    (0..iter).for_each(|x| {
        x + x;
    });
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    println!("Add test, {} ms/iteration", t);
}

//fn scaling_bench() {
//let mut x = Box::new(5);
//let iter = 10;
//for i in ( 0..100000 ).step_by(100) {
//let start = Instant::now();
//for _ in 0..iter {
//x = libuntrusted::safe_do_nothing(x, i);
//}
//let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
//println!("Do nothing {}us, {} ms/iteration", i, t);
//}
//}
