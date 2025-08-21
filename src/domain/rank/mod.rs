pub mod falcon_rank;
pub mod mee6_rank;

#[derive(Debug)]
pub struct Rank {
    pub name: &'static str,
    pub rank_id: u64,
    pub code: &'static str,
}
