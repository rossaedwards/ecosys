#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════
# AURAFS One-Command Kubernetes Deploy
# Aurphyx LLC | Full 3-node cluster + optional Prometheus/Grafana
# Usage: ./scripts/deployk8s.sh [--with-monitoring] [--dry-run]
# ═══════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
K8S_DIR="${REPO_ROOT}/k8s"
HELM_DIR="${K8S_DIR}/helm"
WITH_MONITORING=false
DRY_RUN=false

for arg in "$@"; do
  case "$arg" in
    --with-monitoring) WITH_MONITORING=true ;;
    --dry-run)         DRY_RUN=true ;;
    -h|--help)
      echo "Usage: $0 [--with-monitoring] [--dry-run]"
      echo "  --with-monitoring  Deploy Prometheus + Grafana (requires Helm)"
      echo "  --dry-run         Print manifests only, do not apply"
      exit 0
      ;;
  esac
done

echo "🚀 AuraFS Kubernetes Deploy"
echo "   Repo root: $REPO_ROOT"
echo "   K8s dir:   $K8S_DIR"
echo ""

# Check kubectl
if ! command -v kubectl &>/dev/null; then
  echo "❌ kubectl not found. Install Kubernetes CLI and ensure kubeconfig is set."
  exit 1
fi

# Optional: check cluster connectivity
if ! kubectl cluster-info &>/dev/null; then
  echo "⚠️  Cannot reach cluster (kubectl cluster-info failed). Continue anyway? [y/N]"
  read -r ans
  if [[ "${ans:-n}" != "y" && "${ans:-n}" != "Y" ]]; then
    exit 1
  fi
fi

apply_file() {
  local f="$1"
  if [[ ! -f "$f" ]]; then
    echo "⚠️  Skip (missing): $f"
    return 0
  fi
  if $DRY_RUN; then
    echo "--- dry-run: kubectl apply -f $f"
    kubectl apply -f "$f" --dry-run=client -o yaml | head -5
    echo "   ..."
    return 0
  fi
  echo "📄 Applying $f"
  kubectl apply -f "$f"
}

# 1. Base namespace + API (legacy base)
apply_file "${K8S_DIR}/base.yaml"

# 2. Full production 3-node cluster (aurafs-prod.yaml)
if [[ -f "${HELM_DIR}/aurafs-prod.yaml" ]]; then
  apply_file "${HELM_DIR}/aurafs-prod.yaml"
else
  echo "⚠️  ${HELM_DIR}/aurafs-prod.yaml not found; skipping prod cluster."
fi

# 3. Optional: Prometheus stack (Helm)
if $WITH_MONITORING; then
  if ! command -v helm &>/dev/null; then
    echo "⚠️  Helm not found; skip monitoring. Install Helm for --with-monitoring."
  else
    PROM_VALUES="${K8S_DIR}/prometheus-values.yaml"
    if [[ -f "$PROM_VALUES" ]]; then
      if $DRY_RUN; then
        echo "--- dry-run: helm install prometheus (with $PROM_VALUES)"
      else
        echo "📊 Installing Prometheus (AuraFS shard metrics)"
        helm repo add prometheus-community https://prometheus-community.github.io/helm-charts 2>/dev/null || true
        helm repo update
        helm upgrade --install prometheus prometheus-community/kube-prometheus-stack \
          -f "$PROM_VALUES" \
          -n aurphyx \
          --create-namespace \
          --wait --timeout 5m || true
      fi
    else
      echo "⚠️  $PROM_VALUES not found; skip Prometheus."
    fi
  fi
fi

if $DRY_RUN; then
  echo ""
  echo "✅ Dry-run complete. Run without --dry-run to apply."
  exit 0
fi

echo ""
echo "✅ AuraFS deploy complete."
echo "   Check pods:  kubectl get pods -n aurphyx"
echo "   Check svc:   kubectl get svc -n aurphyx"
echo "   Shard nodes: aurafs-shard-0, aurafs-shard-1, aurafs-shard-2"
echo "   API:         aurafs-api (3 replicas)"
