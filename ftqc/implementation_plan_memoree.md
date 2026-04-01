# Memoree Analysis & Implementation Plan

## Overview & Analysis
I have completely read through the `memoree` directory (`memoree_service.py`, `routes.py`, `memory_engine.py`, `aurafs_backend.py`, `vector_backend.py`, etc.).
Here is my analysis of the current state:

- **Core Functionality**: It's a cohesive "Sovereign Memory" system that effectively double-writes to a JSON file matrix (`AuraFS`) and a local vector database (`ChromaDB`). It bridges legacy SQLite (`Memori`).
- **MCP Server Check**: **No, it is currently NOT an MCP (Model Context Protocol) server.** It is a standard FastAPI REST application exposing JSON endpoints on `127.0.0.1:7042`. It needs the `mcp` Python SDK to expose tools/resources natively to MCP clients.
- **Security**: It is currently local-only, explicitly bound to `127.0.0.1`. There is no authentication or authorization layer.
- **Architecture**: File paths are hardcoded to `~/.aurphyx/memoree`. This makes containerized or cloud deployments tricky unless properly volume-mounted.

## User Review Required

> [!CAUTION]
> **Cloud Deployment Warning**: Serverless cloud services (like Cloud Run or AWS Lambda) are stateless. Because `memoree` relies heavily on local disk persistence (`AuraFS` and local `ChromaDB`), deploying to a "Serverless" cloud requires mounting persistent file storage (e.g. NFS/Filestore or EFS). Conversely, deploying to a VPS (EC2, droplet) with a Docker volume is much simpler and cheaper.

## Proposed Changes & Improvements

### 1. Model Context Protocol (MCP) Integration
Since it isn't an MCP server currently, we need to add a specialized MCP entrypoint.
- Add `mcp` to `requirements.txt`.
- **[NEW] `mcp_server.py`**: A new entrypoint using `mcp.server.Server` (stdio transport) that wraps your `MemoryEngine` methods as MCP Tools.
  - **Tools**: `store_episodic_memory`, `store_semantic_memory`, `read_project_context`, `summarize_thread`.
  - **Resources**: Expose `AuraFS` states or specific summarized memory banks as read-only context resources.

### 2. Security & API Polish (For VPS & Cloud)
If this is to leave `127.0.0.1`, we must lock it down.
- **[MODIFY] `routes.py` and `memoree_service.py`**:
  - Add API Key authentication (via FastAPI `Security` / `APIKeyHeader`).
  - Add CORS middleware so a frontend or web client can access it safely.
- **[MODIFY] `config.yaml` and Path Handling**:
  - Allow environment variables to override the `~/.aurphyx` host paths. This is required for Docker.

### 3. Deployment Packaging (Local, VPS, Cloud)
- **[NEW] `Dockerfile`**: Packages the app and its dependencies (FastAPI + Chroma + SentenceTransformers).
- **[NEW] `docker-compose.yml`**: Wraps the Dockerfile to mount the `~/.aurphyx` paths as a persistent volume. This allows a seamless **Local** and **VPS** deployment using `docker-compose up -d`.
- **Cloud Service Polish**: For cloud deployment, provide instructions or integration for an NGINX reverse-proxy container with Let's Encrypt, or configure it to run on a targeted managed VPS.

## Open Questions

> [!IMPORTANT]
> 1. **MCP Transport**: Do you want the MCP server to run as a **command-line stdio** server (how Claude Desktop/Cursor usually connect to it locally) or as an **SSE (Server-Sent Events) HTTP endpoint** (better for remote Web clients)?
> 2. **Authentication**: For VPS/Cloud deployment, is a simple static `API_KEY` header sufficient for security, or do you need a more complex auth system (OAuth/JWT)?
> 3. **Cloud Strategy**: Because of the local ChromaDB and AuraFS, are you targeting a persistent VPS (like a DigitalOcean Droplet or AWS EC2), or true Serverless (like Google Cloud Run)? A VPS is highly recommended for this architecture.

## Verification Plan

### Automated Tests
- Boot the MCP Server using the MCP Inspector tool to verify that the `MemoryEngine` functions are successfully exposed as MCP tools.
- Run FastAPI tests to verify the API Key protection prevents unauthorized reads/writes.

### Manual Verification
- Deploy via `docker-compose up` and verify that volumes persist across container restarts.
- Attach a local LLM client (like Cursor) to the new MCP server and confirm it can "read memory" and "store memory" autonomously.
