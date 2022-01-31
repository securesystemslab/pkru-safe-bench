#![feature(duration_float)]
use libtrusted;
use libuntrusted;

use clap::{App, Arg};
use serde::Serialize;
use std::error::Error;
use std::io;
use std::fs::File;
use std::time::Instant;
use core::mem::ManuallyDrop;
use std::borrow::BorrowMut;

extern crate rustc_serialize;

#[derive(Debug, Serialize, RustcEncodable)]
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
            Arg::with_name("profile")
                .short("p")
                .long("profile")
                .takes_value(false)
                .help("run profile test 'workload'"),
        )
        .arg(
            Arg::with_name("step")
                .short("s")
                .long("step")
                .takes_value(false)
                .help("run iterations through stepped 'workload'"),
        )
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
    let mut garbage_vec = vec![];

    if matches.is_present("step") {
        access_step_vec_bench(1, 1, 0, &mut garbage_vec);
        let mut step : usize = 1;
        loop {
            access_step_vec_bench(iter, step, 0, &mut res);
            step += 1;
            if step > 200 {
            //if step > 1 {
                break;
            }
        }

        /*
        for inc in (0..11).step_by(1) {
            do_nothing_bench(iter, inc, &mut res);
        }
        for inc in (10..10000).step_by(10) {
            //do_nothing_bench(iter, inc, &mut res);
            //access_mem_bench(iter, inc, &mut res);
            //callback_bench(iter, inc, &mut res);
        }
        for inc in (10000..100001).step_by(10000) {
            //do_nothing_bench(iter, inc, &mut res);
            //access_mem_bench(iter, inc, &mut res);
            //callback_bench(iter, inc, &mut res);
        }
        for inc in (150000..1000001).step_by(50000) {
            //do_nothing_bench(iter, inc, &mut res);
            //access_mem_bench(iter, inc, &mut res);
            //callback_bench(iter, inc, &mut res);
        }
        */
        println!("Step Selected!");
    } else if matches.is_present("profile") {
        access_vec_bench(iter, 0, &mut res);
        access_box_vec_bench(iter, 0, &mut res);
    } else {
        do_nothing_bench(iter, 0, &mut res);
        access_mem_bench(iter, 0, &mut res);
        callback_bench(iter, 0, &mut res);
    }

    for r in res {
        wtr.encode(r)?;
    }

    wtr.flush()?;
    Ok(())
}

fn do_nothing_bench(iter: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x = Box::new(5);
    let start = Instant::now();
    for _ in 0..iter {
        x = libtrusted::safe_do_nothing(x, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        x = libuntrusted::safe_do_nothing(x, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    results.push(Record {
        work: ns,
        uninstrumented: t,
        callgate: u,
    });
    //println!("Do nothing {}us, {}, {}", ns, t, u);
}

fn access_mem_bench(iter: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x = Box::new(5);
    let start = Instant::now();
    for _ in 0..iter {
        libtrusted::safe_read_i32(&mut *x, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        libuntrusted::safe_read_i32(&mut *x, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    results.push(Record {
        work: ns,
        uninstrumented: t,
        callgate: u,
    });
    //println!("Do nothing {}us, {}, {}", ns, t, u);
}

fn callback_bench(iter: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x = Box::new(5);
    let start = Instant::now();
    for _ in 0..iter {
        libtrusted::safe_callback(&mut *x, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        libuntrusted::safe_callback(&mut *x, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    results.push(Record {
        work: ns,
        uninstrumented: t,
        callgate: u,
    });
}

fn into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(v);
    (me.as_mut_ptr(), me.len(), me.capacity())
}

fn access_vec_bench(iter: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x : Vec<i32> = Vec::with_capacity(40960);
    for i in 0..40960 {
        x.push(i);
    }
    let (ptr, len, cap) = into_raw_parts(x);
    let start = Instant::now();
    for _ in 0..iter {
        libtrusted::safe_access_vec(ptr, len, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        libuntrusted::safe_access_vec(ptr, len, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    unsafe {
        let unsafe_vec = Vec::from_raw_parts(ptr, len, cap);
    }

    results.push(Record {
        work: ns,
        uninstrumented: t,
        callgate: u,
    });
}

fn access_step_vec_bench(iter: usize, step: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x : Vec<i32> = Vec::with_capacity(250000000);
    for i in 0..250000000 {
        x.push(i);
    }
    let (ptr, len, cap) = into_raw_parts(x);
    let start = Instant::now();
    for _ in 0..iter {
        libtrusted::safe_access_vec(ptr, step, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        libuntrusted::safe_access_vec(ptr, step, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    unsafe {
        let unsafe_vec = Vec::from_raw_parts(ptr, len, cap);
    }

    results.push(Record {
        work: step,
        uninstrumented: t,
        callgate: u,
    });
}

fn access_box_vec_bench(iter: usize, ns: usize, results: &mut Vec<Record>) {
    let mut x : Vec<Box<i32>> = Vec::with_capacity(40960);
    for i in 0..40960 {
        x.push(Box::new(i));
    }
    let mut y : Vec<*mut i32> = x.iter().map(|inner_box| Box::into_raw(inner_box.to_owned())).collect();
    let (ptr, len, cap) = into_raw_parts(y);
    let start = Instant::now();
    for _ in 0..iter {
        libtrusted::safe_access_box_vec(ptr, len, ns);
    }
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    let start = Instant::now();
    for _ in 0..iter {
        libuntrusted::safe_access_box_vec(ptr, len, ns);
    }
    let u = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;

    unsafe {
        let unsafe_vec = Vec::from_raw_parts(ptr, len, cap);
    }

    results.push(Record {
        work: ns,
        uninstrumented: t,
        callgate: u,
    });
}

#[allow(unused)]
fn add_bench(iter: usize) {
    let start = Instant::now();
    (0..iter).for_each(|x| {
        x + x;
    });
    let t = (start.elapsed().as_secs_f64() / (iter as f64)) * 1000.0;
    println!("Add test, {} ns/iteration", t);
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
//println!("Do nothing {}us, {} ns/iteration", i, t);
//}
//}
