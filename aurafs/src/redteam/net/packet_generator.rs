/// afs/src/redteam/net/packet_generator.rs
/// PACKET_GENERATOR DIAMOND ATTACK
use colored::*;
pub struct PACKET_GENERATOR {
    attacks: u32,
}
impl PACKET_GENERATOR {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "PACKET_GENERATOR".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
