/// afs/src/redteam/chaos/pvp_leaderboard.rs
/// PVP_LEADERBOARD DIAMOND ATTACK
use colored::*;
pub struct PVP_LEADERBOARD {
    attacks: u32,
}
impl PVP_LEADERBOARD {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "PVP_LEADERBOARD".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
