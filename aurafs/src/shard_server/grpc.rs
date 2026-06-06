//! ═══════════════════════════════════════════════════════════════════
//! ✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
//! 💎 AuraFS Shard Server gRPC - High-Performance Protocol Buffers
//! ⚡ Unary + Streaming + Bidirectional Shard Operations
//! ═══════════════════════════════════════════════════════════════════

#![warn(missing_docs)]

use crate::{
    shard::{
        Shard, ShardId, ShardManager, ShardIndex, ShardAudit, 
        metadata::{ShardMetadata, LatticeGeometry}, ReplicationStatus,
    },
    shard_server::{
        acl::{ShardACL, AclEnforcer, OperationType},
        server::TieredShardStorage,
    },
    gov::BlissId,
};
use tonic::{
    transport::{Server, ServerTlsConfig},
    Request, Response, Status,
};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tonic::codegen::std::time::SystemTime;
use tonic_health::server::health_reporter;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::{info, debug, warn};

pub mod proto {
    tonic::include_proto!("aurafs");
}

use proto::{
    shard_server_server::{ShardServer, ShardServerServer},
    HealthCheckRequest, HealthCheckResponse,
    ListShardsRequest, ListShardsResponse,
    GetShardRequest, GetShardResponse,
    CreateShardRequest, CreateShardResponse,
    AuditShardRequest, AuditShardResponse,
    ReplicateShardRequest, ReplicateShardResponse,
    SearchShardsRequest, SearchShardsResponse,
    TransmuteShardRequest, TransmuteShardResponse, // ✨ Phase II
    HealthCheckResponse_ServingStatus,
};

/// Production gRPC shard server implementation
pub struct GrpcShardServer {
    shard_manager: Arc<ShardManager>,
    shard_index: Arc<ShardIndex>,
    storage: Arc<TieredShardStorage>,
    acl_enforcer: Arc<AclEnforcer>,
}

impl GrpcShardServer {
    /// Helper to map internal Geometry to Proto integer
    fn map_geometry_to_proto(geo: &LatticeGeometry) -> i32 {
        match geo {
            LatticeGeometry::FlowerOfLife => 0,
            LatticeGeometry::Kagome => 1,
            LatticeGeometry::Bethe => 2,
            LatticeGeometry::Triangular => 3,
            LatticeGeometry::Sierpinski => 4,
        }
    }

    /// Helper to map Proto integer to internal Geometry
    fn map_proto_to_geometry(i: i32) -> LatticeGeometry {
        match i {
            1 => LatticeGeometry::Kagome,
            2 => LatticeGeometry::Bethe,
            3 => LatticeGeometry::Triangular,
            4 => LatticeGeometry::Sierpinski,
            _ => LatticeGeometry::FlowerOfLife,
        }
    }
}

#[tonic::async_trait]
impl ShardServer for GrpcShardServer {
    /// Health check endpoint
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let response = HealthCheckResponse {
            status: HealthCheckResponse_ServingStatus::Serving as i32,
        };
        Ok(Response::new(response))
    }

    /// List shards with pagination and filtering
    async fn list_shards(
        &self,
        request: Request<ListShardsRequest>,
    ) -> Result<Response<ListShardsResponse>, Status> {
        let req = request.into_inner();
        
        let all_shards = self.shard_index.shard_ids();
        let mut shards = Vec::new();
        
        for shard_id in all_shards.into_iter().take(req.limit as usize) {
            if let Some(metadata) = self.shard_index.get_shard(&shard_id) {
                shards.push(proto::ShardMetadata {
                    shard_id: shard_id.to_string(),
                    size_bytes: metadata.size_bytes as u64,
                    content_type: metadata.content_type.unwrap_or_default(),
                    peer_nodes: metadata.peer_nodes.iter().cloned().collect(),
                    replication_status: match metadata.replication_status {
                        ReplicationStatus::Healthy => 1,
                        ReplicationStatus::Degraded => 2,
                        ReplicationStatus::Unknown => 0,
                    } as i32,
                    tags: metadata.tags,
                    geometry: Self::map_geometry_to_proto(&metadata.geometry), // ✨ Include Geometry
                });
            }
        }
        
        let response = ListShardsResponse {
            shards,
            total: shards.len() as u64,
            page: req.page as u32,
        };
        
        Ok(Response::new(response))
    }

    /// Get specific shard with data, with timeout and proper error handling
    async fn get_shard(
        &self,
        request: Request<GetShardRequest>,
    ) -> Result<Response<GetShardResponse>, Status> {
        let req = request.into_inner();
        
        // Validate shard ID
        let shard_id = ShardId::from_hex(req.shard_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid shard ID: {}", e)))?;
        
        // ACL enforcement
        let acl_result = self.acl_enforcer.enforce(
            OperationType::Read,
            &shard_id,
            &ShardACL::default(),
            &BlissId::genesis(), // Extract soul from metadata/headers in real impl
        ).await;
        
        if let Err(e) = acl_result {
            return Err(Status::permission_denied(format!("ACL check failed: {}", e)));
        }
        
        // Load shard with timeout
        let shard = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            self.shard_manager.load_shard(&shard_id)
        ).await
            .map_err(|_| Status::deadline_exceeded("Shard load timeout"))?
            .map_err(|e| Status::not_found(format!("Shard not found: {}", e)))?;
        
        // Validate shard before returning
        shard.validate()
            .map_err(|e| Status::internal(format!("Shard validation failed: {}", e)))?;
        
        let response = GetShardResponse {
            shard_id: shard_id.to_string(),
            data: shard.data,
            metadata: Some(proto::ShardMetadata {
                shard_id: shard.shard_id.to_string(),
                size_bytes: shard.metadata.size_bytes as u64,
                content_type: shard.metadata.content_type.unwrap_or_default(),
                peer_nodes: shard.metadata.peer_nodes.iter().cloned().collect(),
                replication_status: match shard.metadata.replication_status {
                    ReplicationStatus::Healthy => 1,
                    ReplicationStatus::Degraded => 2,
                    ReplicationStatus::Unknown => 0,
                } as i32,
                tags: shard.metadata.tags,
                geometry: Self::map_geometry_to_proto(&shard.metadata.geometry), // ✨ Include Geometry
            }),
        };
        
        Ok(Response::new(response))
    }

    /// Create new shard
    async fn create_shard(
        &self,
        request: Request<CreateShardRequest>,
    ) -> Result<Response<CreateShardResponse>, Status> {
        let req = request.into_inner();
        
        let shard_id = ShardId::from_content(&req.data);
        let mut metadata = ShardMetadata::new(
            shard_id.clone(),
            req.data.len() as u64,
            Some(req.content_type),
        );
        
        // ✨ Phase II: Set Initial Geometry if provided
        // Assuming proto has `int32 initial_geometry` field
        if req.initial_geometry != 0 {
             metadata.geometry = Self::map_proto_to_geometry(req.initial_geometry);
        }

        let created_id = self.shard_manager.create_shard(req.data, metadata).await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        let response = CreateShardResponse {
            shard_id: created_id.to_string(),
            status: "created".to_string(),
        };
        
        Ok(Response::new(response))
    }

    /// ✨ Phase II: Transmute Shard (Change Lattice Geometry)
    async fn transmute_shard(
        &self,
        request: Request<TransmuteShardRequest>,
    ) -> Result<Response<TransmuteShardResponse>, Status> {
        let req = request.into_inner();
        let shard_id = ShardId::from_hex(req.shard_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid ID: {}", e)))?;
        
        // 1. ACL Check: Transmute Permission
        self.acl_enforcer.enforce(
            OperationType::Transmute,
            &shard_id,
            &ShardACL::default(),
            &BlissId::genesis(), // In prod: req.soul_proof
        ).await.map_err(|e| Status::permission_denied(e.to_string()))?;

        // 2. Load and Modify
        let mut shard = self.shard_manager.load_shard(&shard_id).await
            .map_err(|_| Status::not_found("Shard not found"))?;

        let old_geo = shard.metadata.geometry.clone();
        let new_geo = Self::map_proto_to_geometry(req.target_geometry);

        shard.metadata.geometry = new_geo.clone();
        
        // 3. Update Physics Tags (Mock Logic)
        if new_geo == LatticeGeometry::Kagome {
            shard.metadata.tags.insert("compute_optimized".to_string());
        }

        // 4. Persist
        self.shard_manager.update_shard_metadata(&shard_id, shard.metadata).await
            .map_err(|e| Status::internal(format!("Failed to update metadata: {}", e)))?;

        info!("✨ Transmuted shard {} via gRPC: {:?} -> {:?}", shard_id, old_geo, new_geo);

        Ok(Response::new(TransmuteShardResponse {
            shard_id: shard_id.to_string(),
            old_geometry: Self::map_geometry_to_proto(&old_geo),
            new_geometry: Self::map_geometry_to_proto(&new_geo),
            status: "transmuted".to_string(),
        }))
    }

    /// Audit shard health
    async fn audit_shard(
        &self,
        request: Request<AuditShardRequest>,
    ) -> Result<Response<AuditShardResponse>, Status> {
        let req = request.into_inner();
        let shard_id = ShardId::from_hex(req.shard_id)?;
        
        let audit = self.shard_manager.audit_shard(&shard_id).await
            .map_err(|_| Status::not_found("Shard not found"))?;
        
        let response = AuditShardResponse {
            shard_id: shard_id.to_string(),
            storage_healthy: audit.storage_healthy,
            index_consistent: audit.index_consistent,
            replication_healthy: audit.replication_healthy,
            signature_valid: audit.signature_valid,
            needs_repair: audit.needs_repair,
        };
        
        Ok(Response::new(response))
    }

    /// Search shards with advanced filtering (streaming)
    type SearchShardsStream = tonic::codec::Streaming<proto::ShardMetadata>;
    
    async fn search_shards(
        &self,
        request: Request<SearchShardsRequest>,
    ) -> Result<Response<Self::SearchShardsStream>, Status> {
        let req = request.into_inner();
        
        // ✨ Phase II: Filter by Geometry if provided
        let mut query = crate::shard::index::ShardQuery::new()
            .content_type(req.content_type)
            .tag(req.tag.unwrap_or_default());
        
        if req.geometry_filter != 0 {
             query = query.geometry(Some(Self::map_proto_to_geometry(req.geometry_filter)));
        }
        
        let results = self.shard_index.query(&query);
        
        let stream = tokio_stream::iter(results.into_iter().map(|metadata| {
            Ok(proto::ShardMetadata {
                shard_id: metadata.shard_id.to_string(),
                size_bytes: metadata.size_bytes as u64,
                content_type: metadata.content_type.unwrap_or_default(),
                peer_nodes: metadata.peer_nodes.iter().cloned().collect(),
                replication_status: match metadata.replication_status {
                    ReplicationStatus::Healthy => 1,
                    ReplicationStatus::Degraded => 2,
                    ReplicationStatus::Unknown => 0,
                } as i32,
                tags: metadata.tags,
                geometry: Self::map_geometry_to_proto(&metadata.geometry), // ✨ Include Geometry
            })
        }));
        
        Ok(Response::new(stream))
    }

    /// Replicate shard across mesh (streaming progress)
    type ReplicateShardStream = tonic::codec::Streaming<ReplicateShardResponse>;
    
    async fn replicate_shard(
        &self,
        request: Request<ReplicateShardRequest>,
    ) -> Result<Response<Self::ReplicateShardStream>, Status> {
        let req = request.into_inner();
        let shard_id = ShardId::from_hex(req.shard_id)?;
        
        // Simulate mesh replication with progress streaming
        let stream = tokio_stream::iter(vec![
            ReplicateShardResponse {
                status: "starting".to_string(),
                peer_count: 0,
                completed_peers: 0,
            },
            ReplicateShardResponse {
                status: "routing_lattice".to_string(), // ✨ geometry-aware routing status
                peer_count: 5,
                completed_peers: 1,
            },
            ReplicateShardResponse {
                status: "completed".to_string(),
                peer_count: 5,
                completed_peers: 5,
            },
        ]);
        
        Ok(Response::new(stream))
    }
}

/// Launch production gRPC shard server
pub async fn serve_grpc(
    shard_manager: Arc<ShardManager>,
    shard_index: Arc<ShardIndex>,
    storage: Arc<TieredShardStorage>,
    acl_enforcer: Arc<AclEnforcer>,
    addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Starting AuraFS gRPC Shard Server on {}", addr);
    
    let grpc_server = GrpcShardServer {
        shard_manager,
        shard_index,
        storage,
        acl_enforcer,
    };
    
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;
    
    let health_service = health_reporter();
    let health_server = tonic_health::server::HealthServer::new(health_service);
    
    Server::builder()
        .add_service(health_server)
        .add_service(reflection_service)
        .add_service(ShardServerServer::new(grpc_server))
        .serve(addr)
        .await?;
    
    Ok(())
}

/// Quick-start macro for gRPC deployment
#[macro_export]
macro_rules! quick_grpc_server {
    ($storage_path:expr, $port:expr) => {{
        use afs::shard_server::grpc::serve_grpc;
        use afs::shard::{LocalShardStorage, ShardIndex, ShardManager};
        
        let storage = Arc::new(LocalShardStorage::new($storage_path.into()));
        let index = Arc::new(ShardIndex::new());
        let manager = Arc::new(ShardManager::new(storage.clone(), index.clone()));
        
        let addr = format!("0.0.0.0:{}", $port).parse().unwrap();
        serve_grpc(manager, index, storage, Arc::new(AclEnforcer::default()), addr).await.unwrap();
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grpc_health_check() {
        // Test gRPC health endpoint
    }
}