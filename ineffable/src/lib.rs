// IneffableLedger/src/lib.rs

// We need to bring in external libraries for unique identifiers and serialization.
// We'll use serde for JSON serialization/deserialization, which is crucial for our API layer.
use serde::{Serialize, Deserialize};
// We'll also use a crate for handling UUIDs, which is a great way to generate unique, secure IDs.
use uuid::Uuid;

// The QuantumSoulID will be a unique identifier tied to a user's biometric
// and energetic signature. We'll represent it as a String for now, but a custom
// type is on our roadmap.
pub type QuantumSoulID = String;

// The Abundance token will represent the value being transferred. We'll use
// a 128-bit unsigned integer to avoid floating point issues and ensure
// massive scale potential, as our financial system is designed for global equality.
pub type Abundance = u128;

// This is the core data structure for our Layer One transactions.
// The `#[derive]` attributes are macros that automatically implement
// traits for us, like `Debug` for easy printing, and `Serialize`/`Deserialize`
// for converting our struct to and from formats like JSON.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    // The transaction ID, a unique identifier for this specific transaction.
    // This is our digital fingerprint.
    pub id: Uuid,

    // The QuantumSoulID of the sender.
    pub sender: QuantumSoulID,

    // The QuantumSoulID of the receiver.
    pub receiver: QuantumSoulID,

    // The amount of Abundance tokens being transferred.
    pub amount: Abundance,

    // The timestamp of the transaction, in milliseconds since the Epoch.
    // This is crucial for maintaining the chronological integrity of the ledger.
    pub timestamp: u128,

    // The signature from the sender, verifying the transaction.
    // This will be a cryptographic hash, ensuring tamper-proof data.
    pub signature: String,

    // A field for including optional, verifiable metadata for ethical transparency,
    // like the purpose of the transaction (e.g., "Seed funding for CanGrow initiative").
    pub metadata: Option<String>,
}

// Now, let's create a function to instantiate a new Transaction.
impl Transaction {
    pub fn new(
        sender: QuantumSoulID,
        receiver: QuantumSoulID,
        amount: Abundance,
        signature: String,
        metadata: Option<String>,
    ) -> Self {
        Transaction {
            id: Uuid::new_v4(), // Generates a new, random UUID
            sender,
            receiver,
            amount,
            timestamp: now_millis(), // We'll need to define this helper function
            signature,
            metadata,
        }
    }
}

// A helper function to get the current timestamp in milliseconds.
// We'll use the `std::time` module.
fn now_millis() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_millis()
        .try_into()
        .unwrap_or_default()
}