/// afs/src/redteam/net/arp_poison.rs
/// ARP_POISON DIAMOND ATTACK
use colored::*;
pub struct ARP_POISON {
    attacks: u32,
}
impl ARP_POISON {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "ARP_POISON".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
