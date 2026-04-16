#!/usr/bin/env bash
# install_docker.sh — Instala Docker Engine + Docker Compose no Ubuntu
set -euo pipefail

echo "================================================"
echo " HL Unification — Instalação do Docker"
echo "================================================"
echo ""

# ── 1. Verificar se já está instalado ────────────────────
if command -v docker &>/dev/null; then
    echo "[OK] Docker já está instalado: $(docker --version)"
    if docker compose version &>/dev/null; then
        echo "[OK] Docker Compose já está instalado: $(docker compose version)"
    fi
    echo ""
    echo "Nada a fazer. Para forçar reinstalação, remova o Docker primeiro:"
    echo "  sudo apt remove docker-ce docker-ce-cli containerd.io"
    exit 0
fi

# ── 2. Remover versões antigas (se houver) ────────────────
echo "[1/5] Removendo versões antigas do Docker (se houver)..."
sudo apt-get remove -y docker docker-engine docker.io containerd runc 2>/dev/null || true
echo "      OK"

# ── 3. Dependências para adicionar o repositório ─────────
echo ""
echo "[2/5] Instalando dependências..."
sudo apt-get update -q
sudo apt-get install -y \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
echo "      OK"

# ── 4. Adicionar repositório oficial do Docker ────────────
echo ""
echo "[3/5] Adicionando repositório oficial do Docker..."
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
    | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
  https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" \
  | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

echo "      OK"

# ── 5. Instalar Docker Engine + Compose ──────────────────
echo ""
echo "[4/5] Instalando Docker Engine e Docker Compose..."
sudo apt-get update -q
sudo apt-get install -y \
    docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin
echo "      OK"

# ── 6. Adicionar usuário ao grupo docker ──────────────────
echo ""
echo "[5/5] Adicionando '$USER' ao grupo docker..."
sudo usermod -aG docker "$USER"
echo "      OK"

# ── Verificação final ─────────────────────────────────────
echo ""
echo "================================================"
echo " Docker instalado com sucesso!"
echo "================================================"
echo ""
docker --version
docker compose version
echo ""
echo "IMPORTANTE: Faça logout e login novamente (ou rode"
echo "  'newgrp docker') para usar Docker sem sudo."
echo ""
echo "Para buildar a imagem do projeto:"
echo "  cd $(dirname "$(readlink -f "$0")")/.."
echo "  docker compose -f docker/docker-compose.yml build"
echo ""
