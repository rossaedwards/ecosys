/// afs/src/redteam/net/slowloris.rs
/// SLOWLORIS DIAMOND ATTACK
use colored::*;
pub struct SLOWLORIS {
    attacks: u32,
}
impl SLOWLORIS {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "SLOWLORIS".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
