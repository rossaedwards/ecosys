#!/usr/bin/env bash
set -euo pipefail

echo '🚀 Deploying AuraFS to Kubernetes...'
kubectl apply -f ./k8s/base.yaml
echo '✅ Deploy complete.'
