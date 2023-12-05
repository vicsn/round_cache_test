use std::collections::{BTreeMap, HashSet};

use crate::{trait_module::RoundCache, QUORUM};

pub struct RoundCacheV2 {
    last_highest_round_with_quorum: u64,
    highest_rounds: BTreeMap<u64, HashSet<Vec<u8>>>,
    address_indices: BTreeMap<Vec<u8>, u64>,
}

impl RoundCache for RoundCacheV2 {
    fn id(&self) -> u8 {
        2
    }

    fn new() -> RoundCacheV2 {
        RoundCacheV2 {
            last_highest_round_with_quorum: 0,
            highest_rounds: BTreeMap::new(),
            address_indices: BTreeMap::new(),
        }
    }

    fn update(&mut self, round: u64, address: &Vec<u8>) -> u64 {
        let mut inserted = false;
        if round > self.last_highest_round_with_quorum {
            if let Some(previous_round) = self.address_indices.get(address) {
                if previous_round < &round {
                    self.highest_rounds
                        .get_mut(previous_round)
                        .unwrap()
                        .remove(address);
                    self.highest_rounds
                        .entry(round)
                        .or_insert(HashSet::new())
                        .insert(address.clone());
                    self.address_indices.insert(address.clone(), round);
                    inserted = true;
                }
            } else {
                self.highest_rounds
                    .entry(round)
                    .or_insert(HashSet::new())
                    .insert(address.clone());
                self.address_indices.insert(address.clone(), round);
                inserted = true;
            }
        }

        // Check if we reached quorum on a new round
        if inserted {
            // TODO: is there any better way to do this?
            // - idea 1: keep doubling the increase
            while self.increased_round_count() >= QUORUM {
                self.last_highest_round_with_quorum += 1;
            }
        }
        self.last_highest_round_with_quorum
    }

    fn increased_round_count(&self) -> usize {
        let mut count = 0usize;
        for (_, addresses) in self
            .highest_rounds
            .range(self.last_highest_round_with_quorum + 1..)
        {
            count += addresses.len();
        }
        count
    }

    fn last_highest_round(&self) -> u64 {
        self.last_highest_round_with_quorum
    }
}