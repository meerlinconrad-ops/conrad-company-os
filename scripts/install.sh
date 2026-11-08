#!/usr/bin/env bash
set -e

echo "=========================================================="
echo "⚡ Conrad OS: 1-Click Installer (Rust CRM + Qwen 1.5B)"
echo "=========================================================="

# 1. Check or install Ollama
if ! command -v ollama &> /dev/null; then
    echo "📦 Ollama nicht gefunden. Installiere Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
else
    echo "✅ Ollama ist bereits installiert."
fi

# 2. Start Ollama and pull Qwen 1.5B
echo "🤖 Ziehe lokales Modell: qwen2.5:1.5b..."
ollama serve > /dev/null 2>&1 &
sleep 2
ollama pull qwen2.5:1.5b

# 3. Build Rust binary
echo "🦀 Kompiliere Conrad OS in Release-Qualität..."
cargo build --release

echo ""
echo "=========================================================="
echo "🎉 Installation erfolgreich abgeschlossen!"
echo "Starte Conrad CRM:"
echo "  ./target/release/conrad-crm"
echo ""
echo "Dashboard & Copilot erreichbar unter: http://localhost:8080"
echo "Native MCP-Schnittstelle unter:       http://localhost:8080/mcp"
echo "=========================================================="
