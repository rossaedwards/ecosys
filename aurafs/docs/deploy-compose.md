# AuraFS Docker Compose Deployment

## Prerequisites
- Docker Engine 24+
- Docker Compose v2

## Quick Start
1. Copy environment template:
   - `cp docker/.env.example docker/.env`
2. Adjust values in `docker/.env` (domain, logging, node id).
3. Build and start:
   - `docker compose -f docker/docker-compose.yml up -d --build`

## TLS + DNS (aurafs.dev)
For production TLS at `aurafs.dev`, enable the edge profile:
- `docker compose -f docker/docker-compose.yml --profile edge up -d`

This uses Caddy to terminate TLS and reverse-proxy to `aurafs-gateway:8080`.

## Data Paths
Volumes are used for node data and logs:
- `aurafs_gateway_data` and `aurafs_gateway_logs`
- `aurafs_shard1_data`, `aurafs_shard2_data`, `aurafs_shard3_data`

## Health Checks
Containers are monitored via `pgrep` health checks. Ensure the `aurad`
process remains healthy for orchestration.
