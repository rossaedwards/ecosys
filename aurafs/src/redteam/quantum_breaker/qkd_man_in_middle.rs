/// afs/src/redteam/quantum_breaker/qkd_man_in_middle.rs
/// QKD_MAN_IN_MIDDLE DIAMOND ATTACK
use colored::*;
pub struct QKD_MAN_IN_MIDDLE {
    attacks: u32,
}
impl QKD_MAN_IN_MIDDLE {
    pub async fn attack(&mut self) -> u32 {
        println!("💎 {} ATTACK! {}", "QKD_MAN_IN_MIDDLE".bright_red().bold(), self.attacks);
        self.attacks += 42;
        self.attacks
    }
}
