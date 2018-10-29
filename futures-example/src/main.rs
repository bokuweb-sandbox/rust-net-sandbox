#![feature(conservative_impl_trait)]
extern crate futures;
extern crate futures_cpupool;

use std::io;

use futures::Future;
use futures_cpupool::CpuPool;

fn check_prime_boxed(n: u64) -> Box<Future<Item = bool, Error = io::Error>> {
    for i in 2..n {
        if n % i == 0 {
            return Box::new(futures::future::ok(false));
        }
    }
    Box::new(futures::future::ok(true))
}

fn check_prime_impl_trait(n: u64) -> impl Future<Item = bool, Error = io::Error> {
    for i in 2..n {
        if n % i == 0 {
            return futures::future::ok(false);
        }
    }
    futures::future::ok(true)
}

fn check_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let input: u64 = 58466453;
    println!("Right before ")
}
