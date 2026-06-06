#!/usr/bin/env python3
"""
══════════════════════════════════════════════════════════════
✨ [:: f0rg3d with l0v3 by Aurphyx Quantum Division ::] ✨
💎 AuraFS RNS Daemon - Production Service Wrapper
🌐 Windows Service + Linux Daemon + Process Lifecycle
══════════════════════════════════════════════════════════════

This script wraps 'rns_bridge.py' to run as a background service.
- Windows: Installs as a native Service (Auto-start)
- Linux/MacOS: Runs as a managed background process
"""

import sys
import os
import time
import logging
import traceback
import signal
from pathlib import Path
from threading import Thread

# Setup Paths
CURRENT_DIR = Path(__file__).parent.absolute()
sys.path.insert(0, str(CURRENT_DIR))

# Import the Bridge Core
try:
    from rns_bridge import RNSBridge, shutdown_event
except ImportError as e:
    print(f"❌ CRITICAL: Could not import rns_bridge: {e}")
    sys.exit(1)

# Logging Setup (Daemon-specific)
LOG_FILE = Path.home() / ".aurafs" / "rns_daemon.log"
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [DAEMON] %(message)s",
    handlers=[
        logging.FileHandler(LOG_FILE),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger("rns_daemon")

# ═══════════════════════════════════════════════════════════════
# WINDOWS SERVICE LOGIC
# ═══════════════════════════════════════════════════════════════

try:
    import win32serviceutil
    import win32service
    import win32event
    import servicemanager
    import socket
    
    class AuraFSRNS_Service(win32serviceutil.ServiceFramework):
        _svc_name_ = "AuraFSRNSBridge"
        _svc_display_name_ = "AuraFS Reticulum Bridge"
        _svc_description_ = "Provides Reticulum Mesh connectivity for AuraFS nodes."

        def __init__(self, args):
            win32serviceutil.ServiceFramework.__init__(self, args)
            self.hWaitStop = win32event.CreateEvent(None, 0, 0, None)
            socket.setdefaulttimeout(60)
            self.bridge = None

        def SvcStop(self):
            self.ReportServiceStatus(win32service.SERVICE_STOP_PENDING)
            win32event.SetEvent(self.hWaitStop)
            logger.info("🛑 Windows Service stopping...")
            if self.bridge:
                self.bridge.shutdown()
            shutdown_event.set()

        def SvcDoRun(self):
            servicemanager.LogMsg(
                servicemanager.EVENTLOG_INFORMATION_TYPE,
                servicemanager.PYS_SERVICE_STARTED,
                (self._svc_name_, '')
            )
            logger.info("🚀 Windows Service starting...")
            self.main()

        def main(self):
            try:
                self.bridge = RNSBridge()
                # Run bridge in a separate thread so we can handle stop events
                bridge_thread = Thread(target=self.bridge.run)
                bridge_thread.start()
                
                # Wait for stop signal
                win32event.WaitForSingleObject(self.hWaitStop, win32event.INFINITE)
                
                # Cleanup
                logger.info("✅ Windows Service stopped.")
                
            except Exception as e:
                logger.error(f"❌ Service Failure: {e}")
                servicemanager.LogErrorMsg(traceback.format_exc())

    WINDOWS_AVAILABLE = True

except ImportError:
    WINDOWS_AVAILABLE = False


# ═══════════════════════════════════════════════════════════════
# LINUX / FOREGROUND LOGIC
# ═══════════════════════════════════════════════════════════════

def run_foreground():
    """Run as a standard process (for Linux/Docker/Dev)"""
    logger.info("🚀 Starting RNS Daemon (Foreground Mode)...")
    
    def handle_sig(signum, frame):
        logger.info(f"📡 Signal {signum} received")
        shutdown_event.set()

    signal.signal(signal.SIGINT, handle_sig)
    signal.signal(signal.SIGTERM, handle_sig)

    bridge = RNSBridge()
    try:
        bridge.run()
    except Exception as e:
        logger.error(f"❌ Daemon Crashed: {e}")
        sys.exit(1)

# ═══════════════════════════════════════════════════════════════
# ENTRY POINT
# ═══════════════════════════════════════════════════════════════

if __name__ == '__main__':
    if WINDOWS_AVAILABLE:
        if len(sys.argv) == 1:
            # If run with no args on Windows, try running the service manager
            # or fallback to foreground if not installed as service
            try:
                servicemanager.Initialize()
                servicemanager.PrepareToHostSingle(AuraFSRNS_Service)
                servicemanager.StartServiceCtrlDispatcher()
            except Exception:
                # Fallback for debugging
                run_foreground()
        else:
            # Handle 'install', 'start', 'stop' args
            win32serviceutil.HandleCommandLine(AuraFSRNS_Service)
    else:
        # Linux / MacOS
        run_foreground()