#!/usr/bin/env python3
"""
══════════════════════════════════════════════════════════════
✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
💎 AuraFS Reticulum Network Stack Bridge - Production Python Bridge
🌐 RNS Integration + ZeroMQ Communication + Auto-Restart + Resource Management
══════════════════════════════════════════════════════════════

This Python script bridges AuraFS (Rust) with Reticulum Network Stack (RNS).
It manages Identity, listens for incoming packets AND large resources (shards),
and communicates with the Rust core via ZeroMQ.
"""

import sys
import os
import json
import signal
import time
import logging
import traceback
from pathlib import Path
from typing import Optional, Dict, Any
from threading import Event, Thread
import zmq
import zmq.asyncio

# Try to import RNS - fail gracefully with helpful error message
try:
    import RNS
    RNS_AVAILABLE = True
except ImportError:
    RNS_AVAILABLE = False
    print("⚠️  WARNING: Reticulum Network Stack (RNS) not installed!")
    print("   Install it with: pip install rns")
    print("   Or visit: https://reticulum.network/")
    sys.exit(1)


# ═══════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════

# ZeroMQ socket endpoints (IPC on Unix, TCP on Windows)
ZMQ_SEND_ENDPOINT = "ipc:///tmp/aurafs_rns_send" if os.name != 'nt' else "tcp://127.0.0.1:5555"
ZMQ_RECV_ENDPOINT = "ipc:///tmp/aurafs_rns_recv" if os.name != 'nt' else "tcp://127.0.0.1:5556"

# RNS configuration directory
RNS_CONFIG_DIR = Path.home() / ".reticulum"
RNS_IDENTITY_FILE = RNS_CONFIG_DIR / "aurafs_identity"

# Logging configuration
LOG_FORMAT = "%(asctime)s [%(levelname)s] %(name)s: %(message)s"
logging.basicConfig(
    level=logging.INFO,
    format=LOG_FORMAT,
    handlers=[
        logging.StreamHandler(sys.stdout),
        logging.FileHandler(Path.home() / ".aurafs" / "rns_bridge.log"),
    ]
)
logger = logging.getLogger("rns_bridge")

# Graceful shutdown event
shutdown_event = Event()


# ═══════════════════════════════════════════════════════════════
# RNS PACKET HANDLER (For Metadata & Chat)
# ═══════════════════════════════════════════════════════════════

class RNSBridgePacketHandler:
    """Handles incoming RNS packets (small data) and forwards to Rust"""
    
    def __init__(self, zmq_socket: zmq.Socket):
        self.zmq_socket = zmq_socket
        self.packet_count = 0
        self.error_count = 0
        self.last_packet_time = None
        
    def packet_handler(self, packet: RNS.Packet) -> None:
        """Called when a small RNS packet is received"""
        try:
            self.packet_count += 1
            self.last_packet_time = time.time()
            
            # Extract packet information
            packet_data = {
                "type": "rns_packet_received",
                "destination_hash": packet.destination_hash.hex() if packet.destination_hash else None,
                "source_hash": packet.source_hash.hex() if packet.source_hash else None,
                "data": packet.data.hex() if packet.data else b"".hex(),
                "data_len": len(packet.data) if packet.data else 0,
                "hop_count": packet.hops if hasattr(packet, 'hops') else 0,
                "timestamp": time.time(),
                "sequence": self.packet_count,
            }
            
            # Send to Rust core via ZeroMQ
            message = json.dumps(packet_data).encode('utf-8')
            self.zmq_socket.send(message, zmq.NOBLOCK)
            
            logger.info(f"📦 RNS packet #{self.packet_count} received and forwarded to Rust core "
                        f"({packet_data['data_len']} bytes)")
            
        except zmq.Again:
            logger.warning("⚠️  ZeroMQ send buffer full, dropping packet")
            self.error_count += 1
        except Exception as e:
            logger.error(f"❌ Error handling RNS packet: {e}")
            self.error_count += 1


# ═══════════════════════════════════════════════════════════════
# RNS BRIDGE CORE (Upgraded for Shard Resources)
# ═══════════════════════════════════════════════════════════════

class RNSBridge:
    """
    Main RNS bridge implementation.
    Includes 'Data Slayer' Resource logic for heavy AuraFS Shards.
    """
    
    def __init__(self):
        self.identity: Optional[RNS.Identity] = None
        self.destination: Optional[RNS.Destination] = None
        self.rns_context: Optional[Any] = None
        self.zmq_context: Optional[zmq.Context] = None
        self.zmq_send_socket: Optional[zmq.Socket] = None  # Send to Rust
        self.zmq_recv_socket: Optional[zmq.Socket] = None  # Receive from Rust
        self.packet_handler: Optional[RNSBridgePacketHandler] = None
        
        # [NEW] Link Management for Heavy Resources
        self.links = {}  # destination_hash_hex -> RNS.Link
        
        self.stats = {
            "packets_received": 0,
            "packets_sent": 0,
            "resources_sent": 0,  # [NEW]
            "resources_received": 0, # [NEW]
            "errors": 0,
            "start_time": None,
        }
        
    def initialize(self) -> bool:
        """Initialize RNS and ZeroMQ connections"""
        try:
            logger.info("🔧 Initializing RNS Bridge (Resource Enabled)...")
            
            # Create AuraFS config directory
            Path(Path.home() / ".aurafs").mkdir(exist_ok=True)
            RNS_CONFIG_DIR.mkdir(parents=True, exist_ok=True)
            
            # Initialize RNS
            if not hasattr(RNS, 'Reticulum'):
                logger.error("❌ RNS module structure unexpected. Is RNS installed correctly?")
                return False
                
            self.rns_context = RNS.Reticulum()
            
            # Load or create Identity
            if RNS_IDENTITY_FILE.exists():
                try:
                    with open(RNS_IDENTITY_FILE, "rb") as f:
                        identity_data = f.read()
                    self.identity = RNS.Identity.from_bytes(identity_data)
                    logger.info("✅ Loaded existing RNS Identity")
                except Exception as e:
                    logger.warning(f"⚠️  Failed to load identity, creating new one: {e}")
                    self.identity = RNS.Identity()
                    with open(RNS_IDENTITY_FILE, "wb") as f:
                        f.write(self.identity.get_private_key())
            else:
                self.identity = RNS.Identity()
                with open(RNS_IDENTITY_FILE, "wb") as f:
                    f.write(self.identity.get_private_key())
                logger.info("✅ Created new RNS Identity")
            
            logger.info(f"🆔 RNS Identity Hash: {self.identity.hash.hex()[:16]}...")
            
            # Create destination for receiving packets AND Resources
            self.destination = RNS.Destination(
                self.identity,
                RNS.Destination.OUT,
                RNS.Destination.SINGLE,
                "aurafs",
                "node"
            )
            
            # [NEW] Set callback for incoming Links (needed for receiving Shards)
            self.destination.set_link_established_callback(self._incoming_link_established)
            
            # Register packet handler for small messages
            self.packet_handler = RNSBridgePacketHandler(None) # Socket set later
            self.destination.set_packet_handler(self.packet_handler.packet_handler)
            
            logger.info(f"📡 RNS destination registered: {self.destination.hash.hex()}")
            
            # Initialize ZeroMQ sockets
            self._init_zmq()
            
            # Update packet handler with socket
            if self.packet_handler:
                self.packet_handler.zmq_socket = self.zmq_send_socket
            
            self.stats["start_time"] = time.time()
            logger.info("✅ RNS Bridge initialized successfully")
            return True
            
        except Exception as e:
            logger.error(f"❌ Failed to initialize RNS Bridge: {e}")
            logger.error(traceback.format_exc())
            return False

    def _init_zmq(self) -> None:
        """Initialize ZeroMQ sockets"""
        try:
            self.zmq_context = zmq.Context()
            self.zmq_send_socket = self.zmq_context.socket(zmq.PUSH)
            self.zmq_recv_socket = self.zmq_context.socket(zmq.PULL)
            
            if os.name == 'nt':
                # Windows
                self.zmq_send_socket.bind(ZMQ_SEND_ENDPOINT)
                self.zmq_recv_socket.bind(ZMQ_RECV_ENDPOINT)
            else:
                # Unix
                # Ensure directories exist
                os.makedirs(os.path.dirname(ZMQ_SEND_ENDPOINT.replace("ipc://", "")), exist_ok=True)
                os.makedirs(os.path.dirname(ZMQ_RECV_ENDPOINT.replace("ipc://", "")), exist_ok=True)
                
                # Clean up old sockets
                for endpoint in [ZMQ_SEND_ENDPOINT, ZMQ_RECV_ENDPOINT]:
                    socket_path = endpoint.replace("ipc://", "")
                    if os.path.exists(socket_path):
                        os.unlink(socket_path)
                
                self.zmq_send_socket.bind(ZMQ_SEND_ENDPOINT)
                self.zmq_recv_socket.bind(ZMQ_RECV_ENDPOINT)
                
                # Set permissions (0600)
                os.chmod(ZMQ_SEND_ENDPOINT.replace("ipc://", ""), 0o600)
                os.chmod(ZMQ_RECV_ENDPOINT.replace("ipc://", ""), 0o600)
            
            # Set socket options for reliability
            self.zmq_send_socket.setsockopt(zmq.LINGER, 1000)
            self.zmq_recv_socket.setsockopt(zmq.LINGER, 1000)
            self.zmq_send_socket.setsockopt(zmq.SNDHWM, 1000)
            self.zmq_recv_socket.setsockopt(zmq.RCVHWM, 1000)
            
            logger.info(f"✅ ZeroMQ sockets initialized")
            
        except Exception as e:
            logger.error(f"❌ Failed to initialize ZeroMQ sockets: {e}")
            raise

    # ═══════════════════════════════════════════════════════════
    # [NEW] RESOURCE & LINK MANAGEMENT LOGIC
    # ═══════════════════════════════════════════════════════════

    def _incoming_link_established(self, link):
        """Called when a remote node connects to us (e.g. to send a Shard)"""
        logger.info(f"🔗 Incoming Link established from {link.destination.hash.hex()}")
        # We attach the resource callback to this specific link
        link.set_resource_concluded_callback(self._resource_concluded)

    def _resource_concluded(self, resource):
        """Called when a file (Shard) has finished downloading"""
        try:
            if resource.status == RNS.Resource.COMPLETE:
                logger.info(f"💾 Shard Received! Saved to: {resource.file}")
                self.stats["resources_received"] += 1
                
                # Notify Rust Core about the new file
                msg = {
                    "type": "shard_received",
                    "file_path": str(resource.file),
                    "sender_hash": resource.link.destination.hash.hex(),
                    "size": resource.total_size,
                    "timestamp": time.time()
                }
                self.zmq_send_socket.send(json.dumps(msg).encode('utf-8'), zmq.NOBLOCK)
            else:
                logger.warning(f"⚠️  Shard transfer failed or cancelled. Status: {resource.status}")
        except Exception as e:
            logger.error(f"❌ Error handling received resource: {e}")
            self.error_count += 1

    def _get_or_create_link(self, dest_hash_bytes):
        """Helper to get an existing active link or create a new one"""
        dest_hex = dest_hash_bytes.hex()
        
        # Check for existing active link
        if dest_hex in self.links:
            if self.links[dest_hex].status == RNS.Link.ACTIVE:
                return self.links[dest_hex]
            else:
                # Link is dead/closed, remove it
                del self.links[dest_hex]
        
        # Check if we know this identity
        if not RNS.Identity.validate_announce(dest_hash_bytes):
            logger.info(f"🔎 Identity {dest_hex[:16]}... unknown, requesting path...")
            RNS.Transport.request_path(dest_hash_bytes)
            # In a full async system, we might want to wait here or queue the action.
            # For now, we proceed, and if path is unknown, RNS will try to find it.
            
        # Recall identity
        remote_identity = RNS.Identity.recall(dest_hash_bytes)
        if not remote_identity:
            logger.error(f"❌ Cannot recall Identity for {dest_hex[:16]}...")
            return None
            
        # Create Destination object
        destination = RNS.Destination(
            remote_identity,
            RNS.Destination.OUT,
            RNS.Destination.SINGLE,
            "aurafs",
            "node"
        )
        
        # Establish Link
        logger.info(f"🔗 Establishing new link to {dest_hex[:16]}...")
        link = RNS.Link(destination)
        link.start()
        self.links[dest_hex] = link
        return link

    # ═══════════════════════════════════════════════════════════
    # MESSAGE HANDLING (Rust <-> Python)
    # ═══════════════════════════════════════════════════════════

    def _handle_rust_messages(self) -> None:
        """Thread function to handle messages from Rust core"""
        if not self.zmq_recv_socket:
            return
            
        poller = zmq.Poller()
        poller.register(self.zmq_recv_socket, zmq.POLLIN)
        
        logger.info("🔄 Rust message handler thread started")
        
        while not shutdown_event.is_set():
            try:
                socks = dict(poller.poll(100)) # 100ms timeout
                
                if self.zmq_recv_socket in socks and socks[self.zmq_recv_socket] == zmq.POLLIN:
                    message = self.zmq_recv_socket.recv(zmq.NOBLOCK)
                    try:
                        data = json.loads(message.decode('utf-8'))
                        self._process_rust_message(data)
                    except Exception as e:
                        logger.error(f"❌ Error processing Rust message: {e}")
                        self.stats["errors"] += 1
                        
            except zmq.Again:
                continue
            except Exception as e:
                logger.error(f"❌ Error in message loop: {e}")
                time.sleep(1)

    def _process_rust_message(self, data: Dict[str, Any]) -> None:
        """Dispatch Rust messages to appropriate RNS actions"""
        try:
            msg_type = data.get("type", "unknown")
            
            # --- 1. SEND PACKET (Chat/Metadata/Signaling) ---
            if msg_type == "send_packet":
                dest_hash = bytes.fromhex(data.get("destination_hash"))
                packet_data = bytes.fromhex(data.get("data"))
                
                self._send_rns_packet(dest_hash, packet_data)
                self.stats["packets_sent"] += 1
                
            # --- 2. [NEW] SEND SHARD RESOURCE (Heavy Data) ---
            elif msg_type == "send_shard_resource":
                dest_hash = bytes.fromhex(data.get("destination_hash"))
                file_path = data.get("file_path")
                
                if not os.path.exists(file_path):
                    logger.error(f"❌ File not found for resource transfer: {file_path}")
                    return

                logger.info(f"💎 Initiating Shard Transfer: {file_path} -> {data.get('destination_hash')[:16]}...")
                
                link = self._get_or_create_link(dest_hash)
                if link:
                    # RNS.Resource handles chunking, encryption, and reliability automatically.
                    # auto_compress=False because shards are likely already compressed/encrypted by AuraFS Rust core.
                    resource = RNS.Resource(file_path, link, auto_compress=False)
                    self.stats["resources_sent"] += 1
                    logger.info(f"🚀 Resource transfer started. Size: {os.path.getsize(file_path)} bytes")
                else:
                    logger.error("❌ Could not establish Link for Shard transfer. Check mesh connectivity.")

            # --- 3. GET STATS ---
            elif msg_type == "get_stats":
                stats_msg = {
                    "type": "stats_response",
                    "stats": self.stats,
                    "rns_identity": self.identity.hash.hex() if self.identity else None,
                    "destination_hash": self.destination.hash.hex() if self.destination else None
                }
                self.zmq_send_socket.send(json.dumps(stats_msg).encode('utf-8'), zmq.NOBLOCK)
                
            else:
                logger.warning(f"⚠️  Unknown message type: {msg_type}")
                
        except Exception as e:
            logger.error(f"❌ Logic Error: {e}")
            logger.error(traceback.format_exc())
            self.stats["errors"] += 1

    def _send_rns_packet(self, destination_hash: bytes, data: bytes) -> None:
        """Send a standard RNS Packet"""
        try:
            if not self.destination:
                return
            # Create ad-hoc destination wrapper
            dest = RNS.Destination(None, RNS.Destination.OUT, RNS.Destination.SINGLE, "aurafs", "node")
            dest.hash = destination_hash
            packet = RNS.Packet(dest, data)
            packet.send()
            logger.info(f"📤 RNS packet sent to {destination_hash.hex()[:16]}... ({len(data)} bytes)")
        except Exception as e:
            logger.error(f"❌ Failed to send RNS packet: {e}")
            self.stats["errors"] += 1

    def run(self) -> None:
        """Main run loop"""
        try:
            if not self.initialize():
                logger.error("❌ Initialization failed")
                return
            
            # Start Rust message handler
            rust_thread = Thread(target=self._handle_rust_messages, daemon=True)
            rust_thread.start()
            
            logger.info("🚀 RNS Bridge running. Ctrl+C to stop.")
            
            while not shutdown_event.is_set():
                time.sleep(1)
                
                # Periodic logging (every 60s)
                if int(time.time()) % 60 == 0:
                    logger.info(f"📊 Stats: {self.stats}")
                    
        except KeyboardInterrupt:
            logger.info("⏸️  Interrupted by user")
        except Exception as e:
            logger.error(f"❌ Fatal error: {e}")
            traceback.print_exc()
        finally:
            self.shutdown()
    
    def shutdown(self) -> None:
        """Clean shutdown"""
        logger.info("🛑 Shutting down...")
        shutdown_event.set()
        
        # Clean up links
        for link in self.links.values():
            link.teardown()
            
        if self.zmq_send_socket: self.zmq_send_socket.close()
        if self.zmq_recv_socket: self.zmq_recv_socket.close()
        if self.zmq_context: self.zmq_context.term()
        logger.info("✅ Shutdown complete")

# ═══════════════════════════════════════════════════════════════
# AUTO-RESTART & ENTRY POINT
# ═══════════════════════════════════════════════════════════════

def signal_handler(signum, frame):
    logger.info(f"📡 Received signal {signum}")
    shutdown_event.set()

def main():
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    
    max_restarts = 10
    restart_count = 0
    
    while restart_count < max_restarts and not shutdown_event.is_set():
        try:
            bridge = RNSBridge()
            bridge.run()
            break # Normal exit
        except Exception as e:
            restart_count += 1
            logger.error(f"❌ Crash detected ({restart_count}/{max_restarts}): {e}")
            if restart_count < max_restarts:
                time.sleep(5)
            else:
                sys.exit(1)

if __name__ == "__main__":
    main()