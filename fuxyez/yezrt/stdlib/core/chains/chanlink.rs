//! ChainLink - Individual Links in a Chain
//! 
//! Each ChainLink represents a single transformation or operation
//! that can be composed into larger execution chains.

use std::fmt;
use serde::{Serialize, Deserialize};

/// A link in an execution chain
#[derive(Clone)]
pub struct ChainLink<I, O> {
    pub id: String,
    pub name: String,
    transform: Box<dyn Fn(I) -> O + Send + Sync>,
    pub metadata: LinkMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkMetadata {
    pub created_at: u64,
    pub execution_count: usize,
    pub success_count: usize,
    pub failure_count: usize,
    pub total_duration_ms: u128,
}

impl<I, O> ChainLink<I, O> {
    /// Create new chain link
    pub fn new<F>(name: impl Into<String>, transform: F) -> Self
    where
        F: Fn(I) -> O + Send + Sync + 'static,
    {
        Self {
            id: Self::generate_id(),
            name: name.into(),
            transform: Box::new(transform),
            metadata: LinkMetadata {
                created_at: Self::timestamp(),
                execution_count: 0,
                success_count: 0,
                failure_count: 0,
                total_duration_ms: 0,
            },
        }
    }
    
    /// Execute this link
    pub fn execute(&mut self, input: I) -> O {
        use std::time::Instant;
        
        let start = Instant::now();
        let output = (self.transform)(input);
        let duration = start.elapsed().as_millis();
        
        self.metadata.execution_count += 1;
        self.metadata.success_count += 1;
        self.metadata.total_duration_ms += duration;
        
        output
    }
    
    /// Chain this link with another
    pub fn chain<N, F>(self, next: ChainLink<O, N>) -> ChainedLink<I, O, N>
    where
        I: 'static,
        O: 'static,
        N: 'static,
    {
        ChainedLink {
            first: self,
            second: next,
        }
    }
    
    /// Get average execution time
    pub fn avg_duration_ms(&self) -> u128 {
        if self.metadata.execution_count == 0 {
            0
        } else {
            self.metadata.total_duration_ms / self.metadata.execution_count as u128
        }
    }
    
    fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("link_{:x}", timestamp)
    }
    
    fn timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl<I, O> fmt::Debug for ChainLink<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChainLink")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("metadata", &self.metadata)
            .finish()
    }
}

/// Two links chained together
pub struct ChainedLink<I, M, O> {
    first: ChainLink<I, M>,
    second: ChainLink<M, O>,
}

impl<I, M, O> ChainedLink<I, M, O> {
    pub fn execute(&mut self, input: I) -> O {
        let intermediate = self.first.execute(input);
        self.second.execute(intermediate)
    }
}

/// ChainLink with error handling
pub struct FallibleChainLink<I, O, E> {
    pub id: String,
    pub name: String,
    transform: Box<dyn Fn(I) -> Result<O, E> + Send + Sync>,
    pub metadata: LinkMetadata,
}

impl<I, O, E> FallibleChainLink<I, O, E> {
    pub fn new<F>(name: impl Into<String>, transform: F) -> Self
    where
        F: Fn(I) -> Result<O, E> + Send + Sync + 'static,
    {
        Self {
            id: Self::generate_id(),
            name: name.into(),
            transform: Box::new(transform),
            metadata: LinkMetadata {
                created_at: Self::timestamp(),
                execution_count: 0,
                success_count: 0,
                failure_count: 0,
                total_duration_ms: 0,
            },
        }
    }
    
    pub fn execute(&mut self, input: I) -> Result<O, E> {
        use std::time::Instant;
        
        let start = Instant::now();
        let result = (self.transform)(input);
        let duration = start.elapsed().as_millis();
        
        self.metadata.execution_count += 1;
        match &result {
            Ok(_) => self.metadata.success_count += 1,
            Err(_) => self.metadata.failure_count += 1,
        }
        self.metadata.total_duration_ms += duration;
        
        result
    }
    
    fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("flink_{:x}", timestamp)
    }
    
    fn timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chainlink_creation() {
        let link = ChainLink::new("double", |x: i32| x * 2);
        assert_eq!(link.name, "double");
    }

    #[test]
    fn test_chainlink_execute() {
        let mut link = ChainLink::new("add_five", |x: i32| x + 5);
        let result = link.execute(10);
        
        assert_eq!(result, 15);
        assert_eq!(link.metadata.execution_count, 1);
    }

    #[test]
    fn test_chainlink_composition() {
        let link1 = ChainLink::new("double", |x: i32| x * 2);
        let link2 = ChainLink::new("add_ten", |x: i32| x + 10);
        
        let mut chained = link1.chain(link2);
        let result = chained.execute(5);
        
        assert_eq!(result, 20); // (5 * 2) + 10
    }

    #[test]
    fn test_fallible_chainlink() {
        let mut link = FallibleChainLink::new("safe_divide", |x: i32| {
            if x == 0 {
                Err("Division by zero")
            } else {
                Ok(100 / x)
            }
        });
        
        assert!(link.execute(10).is_ok());
        assert!(link.execute(0).is_err());
        assert_eq!(link.metadata.success_count, 1);
        assert_eq!(link.metadata.failure_count, 1);
    }
}
            Ok(())
    }
    
    /// Get total spinon count
    pub fn total_count(&self) -> usize {
        let pool = self.spinons.read().unwrap();
        pool.len()
    }
}