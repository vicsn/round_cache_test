use std::collections::VecDeque;

use crate::{trait_module::RoundCache, NUM_VALIDATORS, QUORUM};

pub struct RoundCacheV3 {
    last_highest_round_with_quorum: u64,
    highest_rounds: VecDeque<(u64, Vec<Vec<u8>>)>, // TODO: could also opt for linked_list
    address_indices: Vec<(Vec<u8>, u64)>,
}

impl RoundCache for RoundCacheV3 {
    fn id(&self) -> u8 {
        3
    }

    fn new() -> RoundCacheV3 {
        RoundCacheV3 {
            last_highest_round_with_quorum: 0,
            highest_rounds: VecDeque::new(),
            address_indices: vec![],
        }
    }

    fn update(&mut self, round: u64, address: &Vec<u8>) -> u64 {
        let mut inserted = false;
        if round > self.last_highest_round_with_quorum {
            match self
                .address_indices
                .binary_search_by(|(a, _)| a.cmp(address))
            {
                Ok(address_index) => {
                    let (_, old_round) = self.address_indices[address_index];
                    if old_round < round {
                        self.address_indices[address_index].1 = round;
                        inserted = true;
                        if let Ok(address_index) = self
                            .highest_rounds
                            .binary_search_by_key(&old_round, |(r, _)| *r)
                        {
                            if let Ok(address_sub_index) =
                                self.highest_rounds[address_index].1.binary_search(&address)
                            {
                                self.highest_rounds[address_index]
                                    .1
                                    .remove(address_sub_index);
                                if self.highest_rounds[address_index].1.len() == 0 {
                                    self.highest_rounds.remove(address_index);
                                }
                                match self
                                    .highest_rounds
                                    .binary_search_by_key(&round, |(r, _)| *r)
                                {
                                    Ok(new_address_index) => self.highest_rounds[new_address_index]
                                        .1
                                        .push(address.clone()),
                                    Err(new_address_index) => self
                                        .highest_rounds
                                        .insert(new_address_index, (round, vec![address.clone()])),
                                }
                            } else {
                                panic!("We should find the address in self.highest_rounds[address_index].1");
                            }
                        } else {
                            panic!("We should find the old_round in self.highest_rounds");
                        }
                    }
                }
                Err(address_index) => {
                    inserted = true;
                    self.address_indices.insert(address_index, (address.clone(), round));
                    if let Ok(address_index) = self.highest_rounds.binary_search_by_key(&round, |(r, _)| *r) {
                        self.highest_rounds[address_index].1.push(address.clone());
                    } else {
                        self.highest_rounds.push_back((round, vec![address.clone()]));
                    }
                }
            }
            assert!(self.address_indices.len() <= NUM_VALIDATORS); 
            assert!(
                self.highest_rounds
                    .iter()
                    .map(|(_, a)| a.len())
                    .sum::<usize>()
                    <= NUM_VALIDATORS
            );
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
        let quorum_index = match self
            .highest_rounds
            .binary_search_by_key(&(self.last_highest_round_with_quorum + 1), |(r, _)| *r) {
                Ok(quorum_index) => quorum_index,
                Err(quorum_index) => quorum_index, // TODO: technically this might be one too low
            };
        for (_, addresses) in self
            .highest_rounds
            .range(quorum_index..)
        {
            count += addresses.len();
        }
        count
    }

    fn last_highest_round(&self) -> u64 {
        self.last_highest_round_with_quorum
    }
}
