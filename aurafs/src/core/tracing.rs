//! ═══════════════════════════════════════════════════════════════════
//! 🔍 AuraFS Core Tracing - Distributed Observability
//! ✨ f0rg3d with Ineffable l0v3 by Aurphyx Quantum Division ✨
//! OpenTelemetry-compatible distributed tracing with:
//! - Span creation and propagation
//! - Context injection/extraction
//! - Sampling strategies
//! - Metrics correlation
//! ═══════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::core::{Result, BlissId};

/// Trace ID (128-bit)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraceId(pub String);

impl TraceId {
    /// Generate new trace ID
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string().replace("-", ""))
    }
    
    /// Create from string
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    
    /// Get as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Span ID (64-bit)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpanId(pub String);

impl SpanId {
    /// Generate new span ID
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string()[..16].to_string().replace("-", ""))
    }
    
    /// Create from string
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for SpanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Span kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanKind {
    /// Internal operation
    Internal,
    /// Server handling request
    Server,
    /// Client making request
    Client,
    /// Message producer
    Producer,
    /// Message consumer
    Consumer,
}

/// Span status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanStatus {
    /// Unset (default)
    Unset,
    /// Successful completion
    Ok,
    /// Error occurred
    Error,
}

/// Span event (log within span)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Event name
    pub name: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Event attributes
    pub attributes: HashMap<String, SpanAttribute>,
}

/// Span attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpanAttribute {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    StringArray(Vec<String>),
}

impl From<String> for SpanAttribute {
    fn from(s: String) -> Self {
        SpanAttribute::String(s)
    }
}

impl From<&str> for SpanAttribute {
    fn from(s: &str) -> Self {
        SpanAttribute::String(s.to_string())
    }
}

impl From<i64> for SpanAttribute {
    fn from(v: i64) -> Self {
        SpanAttribute::Int(v)
    }
}

impl From<f64> for SpanAttribute {
    fn from(v: f64) -> Self {
        SpanAttribute::Float(v)
    }
}

impl From<bool> for SpanAttribute {
    fn from(v: bool) -> Self {
        SpanAttribute::Bool(v)
    }
}

/// Trace context for propagation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    /// Trace ID
    pub trace_id: TraceId,
    /// Parent span ID
    pub parent_span_id: Option<SpanId>,
    /// Trace flags (sampled, etc.)
    pub trace_flags: u8,
    /// Trace state (vendor-specific)
    pub trace_state: HashMap<String, String>,
}

impl TraceContext {
    /// Create new root context
    pub fn new_root() -> Self {
        Self {
            trace_id: TraceId::generate(),
            parent_span_id: None,
            trace_flags: 0x01, // Sampled
            trace_state: HashMap::new(),
        }
    }
    
    /// Create child context
    pub fn child(&self, parent_span_id: SpanId) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            parent_span_id: Some(parent_span_id),
            trace_flags: self.trace_flags,
            trace_state: self.trace_state.clone(),
        }
    }
    
    /// Check if sampled
    pub fn is_sampled(&self) -> bool {
        self.trace_flags & 0x01 != 0
    }
    
    /// Serialize to W3C trace context header
    pub fn to_traceparent(&self) -> String {
        let parent_id = self.parent_span_id
            .as_ref()
            .map(|s| s.0.clone())
            .unwrap_or_else(|| "0000000000000000".to_string());
        
        format!("00-{}-{}-{:02x}", self.trace_id.0, parent_id, self.trace_flags)
    }
    
    /// Parse from W3C trace context header
    pub fn from_traceparent(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split('-').collect();
        if parts.len() != 4 || parts[0] != "00" {
            return None;
        }
        
        let trace_id = TraceId::from_string(parts[1]);
        let parent_span_id = if parts[2] == "0000000000000000" {
            None
        } else {
            Some(SpanId::from_string(parts[2]))
        };
        let trace_flags = u8::from_str_radix(parts[3], 16).ok()?;
        
        Some(Self {
            trace_id,
            parent_span_id,
            trace_flags,
            trace_state: HashMap::new(),
        })
    }
}

/// Span data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Span ID
    pub span_id: SpanId,
    /// Trace context
    pub context: TraceContext,
    /// Span name
    pub name: String,
    /// Span kind
    pub kind: SpanKind,
    /// Start time
    pub start_time: SystemTime,
    /// End time
    pub end_time: Option<SystemTime>,
    /// Duration in microseconds
    pub duration_us: Option<u64>,
    /// Status
    pub status: SpanStatus,
    /// Status message (for errors)
    pub status_message: Option<String>,
    /// Attributes
    pub attributes: HashMap<String, SpanAttribute>,
    /// Events
    pub events: Vec<SpanEvent>,
    /// Soul ID (if applicable)
    pub soul_id: Option<BlissId>,
}

impl Span {
    /// Create new span
    pub fn new(name: impl Into<String>, context: TraceContext, kind: SpanKind) -> Self {
        Self {
            span_id: SpanId::generate(),
            context,
            name: name.into(),
            kind,
            start_time: SystemTime::now(),
            end_time: None,
            duration_us: None,
            status: SpanStatus::Unset,
            status_message: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            soul_id: None,
        }
    }
    
    /// Set attribute
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<SpanAttribute>) {
        self.attributes.insert(key.into(), value.into());
    }
    
    /// Add event
    pub fn add_event(&mut self, name: impl Into<String>) {
        self.events.push(SpanEvent {
            name: name.into(),
            timestamp: SystemTime::now(),
            attributes: HashMap::new(),
        });
    }
    
    /// Add event with attributes
    pub fn add_event_with_attributes(
        &mut self,
        name: impl Into<String>,
        attributes: HashMap<String, SpanAttribute>,
    ) {
        self.events.push(SpanEvent {
            name: name.into(),
            timestamp: SystemTime::now(),
            attributes,
        });
    }
    
    /// Set status to OK
    pub fn set_ok(&mut self) {
        self.status = SpanStatus::Ok;
    }
    
    /// Set status to error
    pub fn set_error(&mut self, message: impl Into<String>) {
        self.status = SpanStatus::Error;
        self.status_message = Some(message.into());
    }
    
    /// Set soul ID
    pub fn set_soul(&mut self, soul_id: BlissId) {
        self.soul_id = Some(soul_id);
        self.set_attribute("soul.id", soul_id.to_hex().to_string());
    }
    
    /// End the span
    pub fn end(&mut self) {
        let end_time = SystemTime::now();
        let duration = end_time.duration_since(self.start_time)
            .unwrap_or(Duration::ZERO);
        
        self.end_time = Some(end_time);
        self.duration_us = Some(duration.as_micros() as u64);
    }
    
    /// Check if span is ended
    pub fn is_ended(&self) -> bool {
        self.end_time.is_some()
    }
    
    /// Get child context
    pub fn child_context(&self) -> TraceContext {
        self.context.child(self.span_id.clone())
    }
}

/// Span exporter trait
#[async_trait::async_trait]
pub trait SpanExporter: Send + Sync {
    /// Export completed spans
    async fn export(&self, spans: Vec<Span>) -> Result<()>;
    
    /// Shutdown exporter
    async fn shutdown(&self);
}

/// Console span exporter (for development)
pub struct ConsoleExporter {
    enabled: bool,
}

impl ConsoleExporter {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

#[async_trait::async_trait]
impl SpanExporter for ConsoleExporter {
    async fn export(&self, spans: Vec<Span>) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        for span in spans {
            let status = match span.status {
                SpanStatus::Ok => "OK",
                SpanStatus::Error => "ERROR",
                SpanStatus::Unset => "UNSET",
            };
            
            let duration = span.duration_us.unwrap_or(0);
            
            println!(
                "[TRACE] {} | {} | {} | {}us | {:?}",
                span.context.trace_id,
                span.name,
                status,
                duration,
                span.kind,
            );
            
            for event in &span.events {
                println!("  [EVENT] {}", event.name);
            }
        }
        
        Ok(())
    }
    
    async fn shutdown(&self) {
        // No-op for console
    }
}

/// Tracer for creating spans
pub struct Tracer {
    service_name: String,
    sample_rate: f64,
    exporters: Vec<Arc<dyn SpanExporter>>,
    pending_spans: Arc<RwLock<Vec<Span>>>,
    enabled: bool,
}

impl Tracer {
    /// Create new tracer
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            sample_rate: 1.0,
            exporters: Vec::new(),
            pending_spans: Arc::new(RwLock::new(Vec::new())),
            enabled: true,
        }
    }
    
    /// Set sample rate (0.0 to 1.0)
    pub fn with_sample_rate(mut self, rate: f64) -> Self {
        self.sample_rate = rate.clamp(0.0, 1.0);
        self
    }
    
    /// Add exporter
    pub fn with_exporter(mut self, exporter: Arc<dyn SpanExporter>) -> Self {
        self.exporters.push(exporter);
        self
    }
    
    /// Enable/disable tracing
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Start a new root span
    pub fn start_span(&self, name: impl Into<String>, kind: SpanKind) -> Option<Span> {
        if !self.enabled || !self.should_sample() {
            return None;
        }
        
        let context = TraceContext::new_root();
        let mut span = Span::new(name, context, kind);
        span.set_attribute("service.name", self.service_name.clone());
        
        Some(span)
    }
    
    /// Start a child span
    pub fn start_child_span(
        &self,
        name: impl Into<String>,
        parent: &Span,
        kind: SpanKind,
    ) -> Option<Span> {
        if !self.enabled {
            return None;
        }
        
        let context = parent.child_context();
        let mut span = Span::new(name, context, kind);
        span.set_attribute("service.name", self.service_name.clone());
        
        Some(span)
    }
    
    /// Start span from context
    pub fn start_span_from_context(
        &self,
        name: impl Into<String>,
        context: TraceContext,
        kind: SpanKind,
    ) -> Option<Span> {
        if !self.enabled || !context.is_sampled() {
            return None;
        }
        
        let mut span = Span::new(name, context, kind);
        span.set_attribute("service.name", self.service_name.clone());
        
        Some(span)
    }
    
    /// Record completed span
    pub async fn record(&self, span: Span) {
        if !self.enabled {
            return;
        }
        
        let mut pending = self.pending_spans.write().await;
        pending.push(span);
        
        // Flush if batch is large enough
        if pending.len() >= 100 {
            let spans: Vec<Span> = pending.drain(..).collect();
            drop(pending);
            self.flush_spans(spans).await;
        }
    }
    
    /// Flush pending spans to exporters
    pub async fn flush(&self) {
        let spans = {
            let mut pending = self.pending_spans.write().await;
            pending.drain(..).collect::<Vec<_>>()
        };
        
        self.flush_spans(spans).await;
    }
    
    async fn flush_spans(&self, spans: Vec<Span>) {
        if spans.is_empty() {
            return;
        }
        
        for exporter in &self.exporters {
            if let Err(e) = exporter.export(spans.clone()).await {
                tracing::warn!("Failed to export spans: {}", e);
            }
        }
    }
    
    /// Shutdown tracer
    pub async fn shutdown(&self) {
        self.flush().await;
        
        for exporter in &self.exporters {
            exporter.shutdown().await;
        }
    }
    
    fn should_sample(&self) -> bool {
        if self.sample_rate >= 1.0 {
            return true;
        }
        if self.sample_rate <= 0.0 {
            return false;
        }
        
        rand::random::<f64>() < self.sample_rate
    }
}

/// Span guard for automatic ending
pub struct SpanGuard<'a> {
    tracer: &'a Tracer,
    span: Option<Span>,
}

impl<'a> SpanGuard<'a> {
    /// Create new guard
    pub fn new(tracer: &'a Tracer, span: Option<Span>) -> Self {
        Self { tracer, span }
    }
    
    /// Get span reference
    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
    
    /// Get mutable span reference
    pub fn span_mut(&mut self) -> Option<&mut Span> {
        self.span.as_mut()
    }
    
    /// Set attribute on span
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<SpanAttribute>) {
        if let Some(span) = &mut self.span {
            span.set_attribute(key, value);
        }
    }
    
    /// Add event to span
    pub fn add_event(&mut self, name: impl Into<String>) {
        if let Some(span) = &mut self.span {
            span.add_event(name);
        }
    }
    
    /// Set error status
    pub fn set_error(&mut self, message: impl Into<String>) {
        if let Some(span) = &mut self.span {
            span.set_error(message);
        }
    }
    
    /// Mark as successful
    pub fn set_ok(&mut self) {
        if let Some(span) = &mut self.span {
            span.set_ok();
        }
    }
}

impl<'a> Drop for SpanGuard<'a> {
    fn drop(&mut self) {
        if let Some(mut span) = self.span.take() {
            span.end();
            // Note: In real implementation, this would need async handling
            // For now, we just log the span
            let service = span.attributes.get("service.name")
                .map(|a| match a {
                    SpanAttribute::String(s) => s.as_str(),
                    _ => "unknown",
                })
                .unwrap_or("unknown");
            
            tracing::debug!(
                trace_id = %span.context.trace_id,
                span_id = %span.span_id,
                service = %service,
                name = %span.name,
                duration_us = ?span.duration_us,
                "Span ended"
            );
        }
    }
}

// ======================================================================
// TESTS
// ======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trace_context_propagation() {
        let ctx = TraceContext::new_root();
        let header = ctx.to_traceparent();
        
        let parsed = TraceContext::from_traceparent(&header).unwrap();
        assert_eq!(ctx.trace_id.0, parsed.trace_id.0);
        assert_eq!(ctx.trace_flags, parsed.trace_flags);
    }
    
    #[test]
    fn test_span_creation() {
        let ctx = TraceContext::new_root();
        let mut span = Span::new("test_operation", ctx, SpanKind::Internal);
        
        span.set_attribute("key", "value");
        span.add_event("event1");
        span.set_ok();
        span.end();
        
        assert!(span.is_ended());
        assert_eq!(span.status, SpanStatus::Ok);
        assert!(span.duration_us.is_some());
    }
    
    #[test]
    fn test_child_span() {
        let ctx = TraceContext::new_root();
        let parent = Span::new("parent", ctx, SpanKind::Server);
        
        let child_ctx = parent.child_context();
        let child = Span::new("child", child_ctx, SpanKind::Internal);
        
        // Same trace ID
        assert_eq!(parent.context.trace_id, child.context.trace_id);
        // Parent span ID set
        assert_eq!(child.context.parent_span_id.as_ref().unwrap(), &parent.span_id);
    }
    
    #[tokio::test]
    async fn test_tracer() {
        let tracer = Tracer::new("test-service")
            .with_sample_rate(1.0)
            .with_exporter(Arc::new(ConsoleExporter::new(false)));
        
        let span = tracer.start_span("test_op", SpanKind::Internal);
        assert!(span.is_some());
        
        let mut span = span.unwrap();
        span.set_attribute("test", "value");
        span.end();
        
        tracer.record(span).await;
        tracer.flush().await;
    }
}
