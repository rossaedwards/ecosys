/// afs/src/redteam/net/icmp_death.rs
/// ICMP_DEATH DIAMOND ATTACK
use colored::*;
pub struct ICMP_DEATH {
    attacks: u32,
}
impl ICMP_DEATH {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "ICMP_DEATH".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
