#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "🌐 Conrad OS: Domain & Routing Setup (crm.conrad-ki.com)"
echo "=========================================================="

MODE="${1:-cloudflare}"

if [ "$MODE" = "cloudflare" ]; then
    echo "1️⃣  Prüfe Cloudflare Origin-Zertifikat..."
    if [ ! -f "$HOME/.cloudflared/cert.pem" ]; then
        echo "🔑 Bitte einmalig im Browser bei Cloudflare authentifizieren:"
        cloudflared tunnel login
    fi

    echo "2️⃣  Erstelle benannten Tunnel 'conrad-crm' (falls noch nicht vorhanden)..."
    cloudflared tunnel list | grep -q "conrad-crm" || cloudflared tunnel create conrad-crm

    echo "3️⃣  Setze DNS CNAME Route für crm.conrad-ki.com..."
    cloudflared tunnel route dns -f conrad-crm crm.conrad-ki.com

    echo "4️⃣  Starte persistenten Tunnel auf Port 8090..."
    cloudflared tunnel --config "$(dirname "$0")/../cloudflare-tunnel.yml" run conrad-crm
elif [ "$MODE" = "caddy" ]; then
    echo "🚀 Starte Caddy Reverse Proxy mit automatischem Let's Encrypt SSL & HTTP/3..."
    caddy run --config "$(dirname "$0")/../Caddyfile"
else
    echo "Verwendung: $0 [cloudflare|caddy]"
    exit 1
fi
