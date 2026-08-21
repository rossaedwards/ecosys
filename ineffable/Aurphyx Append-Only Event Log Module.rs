use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use uuid::Uuid;
use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write, Read};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventPayload {
    pub rtype: String,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerEvent {
    pub id: Uuid,
    pub payload: EventPayload,
    pub prev_hash: Option<[u8; 32]>,
    pub signature: Vec<u8>,
    pub pubkey: Vec<u8>,
}

impl LedgerEvent {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_vec(&self.payload).expect("payload serialization"));
        if let Some(ph) = &self.prev_hash {
            hasher.update(ph);
        }
        hasher.finalize().into()
    }

    pub fn verify_signature(&self) -> Result<()> {
        let pubkey = PublicKey::from_bytes(&self.pubkey)?;
        let sig = Signature::from_bytes(&self.signature)?;
        let payload_bytes = serde_json::to_vec(&self.payload)?;
        pubkey.verify(&payload_bytes, &sig)?;
        Ok(())
    }

    pub fn verify_chain(&self, prev_event: Option<&LedgerEvent>) -> Result<()> {
        if let Some(prev) = prev_event {
            if Some(prev.hash()) != self.prev_hash {
                anyhow::bail!("Event prev_hash does not match previous event hash");
            }
        }
        self.verify_signature()?;
        Ok(())
    }
}

pub struct EventLog {
    file: BufWriter<File>,
    last_hash: Option<[u8; 32]>,
}

impl EventLog {
    pub fn open(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .context("opening log file")?;
        Ok(Self {
            file: BufWriter::new(file),
            last_hash: None,
        })
    }

    pub fn append(&mut self, event: &LedgerEvent) -> Result<()> {
        if self.last_hash != event.prev_hash {
            anyhow::bail!("Event chain broken: previous hash mismatch");
        }
        event.verify_signature()?;
        let json = serde_json::to_vec(event)?;
        self.file.write_all(&json)?;
        self.file.write_all(b"\n")?;
        self.file.flush()?;
        self.last_hash = Some(event.hash());
        Ok(())
    }

    pub fn replay(path: &str) -> Result<Vec<LedgerEvent>> {
        let file = File::open(path).context("opening log file for replay")?;
        let reader = BufReader::new(file);
        let mut prev_hash: Option<[u8; 32]> = None;
        let mut events = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let event: LedgerEvent = serde_json::from_str(&line)?;
            event.verify_chain(events.last()).context("chain verification failed")?;
            prev_hash = Some(event.hash());
            events.push(event);
        }
        Ok(events)
    }
}