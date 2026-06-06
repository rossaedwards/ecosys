/// afs/src/redteam/net/tcp_syn_flood.rs
/// TCP_SYN_FLOOD DIAMOND ATTACK
use colored::*;
pub struct TCP_SYN_FLOOD {
    attacks: u32,
}
impl TCP_SYN_FLOOD {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "TCP_SYN_FLOOD".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
