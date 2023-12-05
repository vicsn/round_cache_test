extern crate rand;

use std::time::Instant;

mod trait_module;
use trait_module::RoundCache;
mod v1;
use v1::RoundCacheV1;
mod v2;
use v2::RoundCacheV2;
mod v3;
use v3::RoundCacheV3;

const QUORUM: usize = 134;
const NUM_VALIDATORS: usize = 200;

// This has a theoretical risk that we don't store all of the addresses' rounds
// struct RoundCacheV3 {
// 	last_highest_round_with_quorum: u64,
// 	highest_rounds: VecDeque<HashSet<Vec<u8>>>,
//  address_indices: BTreeMap<Vec<u8>, usize>,
// }

fn run_test_constant_address<T: RoundCache>(mut round_cache: T, addresses: &Vec<Vec<u8>>, num_updates: usize) {
    let now = Instant::now();
    for round in 1..num_updates {
        round_cache.update(round as u64, &addresses[0]);
    }
    let elapsed_time = now.elapsed();
    println!(
        "Updating v{} {} times took {} milliseconds, last round: {}",
        round_cache.id(),
        num_updates,
        elapsed_time.as_millis(),
        round_cache.last_highest_round()
    );
}

fn run_test_increasing_round<T: RoundCache>(mut round_cache: T, addresses: &Vec<Vec<u8>>, num_updates: usize) {
    let now = Instant::now();
    for round in 1..num_updates {
        round_cache.update(round as u64, &addresses[round % 200]);
    }
    let elapsed_time = now.elapsed();
    println!(
        "Updating v{} {} times took {} milliseconds, last round: {}",
        round_cache.id(),
        num_updates,
        elapsed_time.as_millis(),
        round_cache.last_highest_round()
    );
}

fn main() {
    // pre_generate NUM_VALIDATORS random addresses
    let mut addresses = Vec::new();
    for _ in 0..NUM_VALIDATORS {
        let mut address = Vec::new();
        for _ in 0..32 {
            address.push(rand::random::<u8>());
        }
        addresses.push(address);
    }

    let runs = [1_000, 10_000, 100_000];
    for run in runs {
        run_test_increasing_round(RoundCacheV1::new(), &addresses, run);
        run_test_increasing_round(RoundCacheV2::new(), &addresses, run);
        run_test_increasing_round(RoundCacheV3::new(), &addresses, run);
        run_test_constant_address(RoundCacheV1::new(), &addresses, run);
        run_test_constant_address(RoundCacheV2::new(), &addresses, run);
        run_test_constant_address(RoundCacheV3::new(), &addresses, run);
    }
}
