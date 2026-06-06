/// afs/src/redteam/chaos/nodekiller.rs
/// NODEKILLER DIAMOND ATTACK
use colored::*;
pub struct NODEKILLER {
    attacks: u32,
}
impl NODEKILLER {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "NODEKILLER".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
