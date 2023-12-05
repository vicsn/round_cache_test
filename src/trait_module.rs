pub trait RoundCache {
    fn id(&self) -> u8;
    fn new() -> Self;
    fn update(&mut self, round: u64, address: &Vec<u8>) -> u64;
    fn increased_round_count(&self) -> usize;
    fn last_highest_round(&self) -> u64;
}