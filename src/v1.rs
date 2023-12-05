use std::collections::BTreeMap;

use crate::{trait_module::RoundCache, QUORUM};

pub struct RoundCacheV1 {
    last_highest_round_with_quorum: u64,
    highest_rounds: BTreeMap<Vec<u8>, u64>,
}

impl RoundCache for RoundCacheV1 {
    fn id(&self) -> u8 {
        1
    }

    fn new() -> RoundCacheV1 {
        RoundCacheV1 {
            last_highest_round_with_quorum: 0,
            highest_rounds: BTreeMap::new(),
        }
    }

    fn update(&mut self, round: u64, address: &Vec<u8>) -> u64 {
        let mut inserted = false;
        if round > self.last_highest_round_with_quorum {
            if let Some(previous_round) = self.highest_rounds.get(address) {
                if previous_round < &round {
                    self.highest_rounds.insert(address.clone(), round);
                    inserted = true;
                }
            } else {
                self.highest_rounds.insert(address.clone(), round);
                inserted = true;
            }
        }

        // Check if we reached quorum on a new round
        if inserted {
            // TODO: is there any better way to do this?
            // - idea 1: keep doubling the increase
            while self.increased_round_count() >= QUORUM {
                self.last_highest_round_with_quorum += 1;
                // println!("self.last_highest_round_with_quorum: {}", self.last_highest_round_with_quorum);
            }
        }
        self.last_highest_round_with_quorum
    }

    fn increased_round_count(&self) -> usize {
        self.highest_rounds
            .iter()
            .filter(|(_, &round)| round > self.last_highest_round_with_quorum)
            .count()
    }

    fn last_highest_round(&self) -> u64 {
        self.last_highest_round_with_quorum
    }
}