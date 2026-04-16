#!/usr/bin/env bash
# install_docker.sh — Instalação via Docker (Ubuntu 22.04 + ROS2 Humble)
# Instala o Docker se necessário, faz o build da imagem e sobe o ambiente.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "================================================"
echo " HL Unification — Instalação via Docker"
echo "================================================"
echo ""

# ── 1. Verificar Docker ───────────────────────────────────
echo "[1/4] Verificando Docker..."
if ! command -v docker &>/dev/null; then
    echo "      Docker não encontrado. Instalando..."
    echo ""

    # Remover versões antigas
    sudo apt-get remove -y docker docker-engine docker.io containerd runc 2>/dev/null || true

    # Dependências do repositório
    sudo apt-get update -q
    sudo apt-get install -y ca-certificates curl gnupg lsb-release

    # Chave e repositório oficial do Docker
    sudo install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
        | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    sudo chmod a+r /etc/apt/keyrings/docker.gpg

    echo \
      "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
      https://download.docker.com/linux/ubuntu \
      $(lsb_release -cs) stable" \
      | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

    # Instalar
    sudo apt-get update -q
    sudo apt-get install -y \
        docker-ce \
        docker-ce-cli \
        containerd.io \
        docker-buildx-plugin \
        docker-compose-plugin

    # Adicionar usuário ao grupo docker
    sudo usermod -aG docker "$USER"

    echo ""
    echo "      Docker instalado — OK"
    echo ""
    echo "      ATENÇÃO: Seu usuário foi adicionado ao grupo 'docker'."
    echo "      Para aplicar sem reiniciar, rode agora:"
    echo "        newgrp docker"
    echo "      E então re-execute este script:"
    echo "        bash install_docker.sh"
    echo ""
    exit 0
fi
echo "      $(docker --version) — OK"

# Verificar Docker Compose
if ! docker compose version &>/dev/null; then
    echo "[ERRO] Docker Compose plugin não encontrado."
    echo "       Reinstale o Docker com:"
    echo "         sudo apt-get install docker-compose-plugin"
    exit 1
fi
echo "      $(docker compose version) — OK"

# Verificar se consegue rodar sem sudo
if ! docker info &>/dev/null; then
    echo ""
    echo "[ERRO] Não foi possível conectar ao Docker."
    echo "       Se acabou de instalar, rode:"
    echo "         newgrp docker"
    echo "       E re-execute este script."
    exit 1
fi

# ── 2. Verificar contexto de build ───────────────────────
echo ""
echo "[2/4] Verificando arquivos do projeto..."
if [ ! -f "$SCRIPT_DIR/docker/Dockerfile" ]; then
    echo "[ERRO] docker/Dockerfile não encontrado."
    echo "       Certifique-se de rodar este script da raiz do projeto."
    exit 1
fi
echo "      Dockerfile — OK"
echo "      docker-compose.yml — OK"

# ── 3. Build da imagem ────────────────────────────────────
echo ""
echo "[3/4] Fazendo build da imagem Docker..."
echo "      (Ubuntu 22.04 + ROS2 Humble + Booster SDK + Python 3.10)"
echo "      Primeira vez pode levar 10-15 minutos."
echo ""
docker compose -f "$SCRIPT_DIR/docker/docker-compose.yml" build

echo ""
echo "      Build concluído — OK"

# ── 4. Verificar imagem ───────────────────────────────────
echo ""
echo "[4/4] Verificando imagem gerada..."
if docker image inspect hl-unification:humble &>/dev/null; then
    SIZE=$(docker image inspect hl-unification:humble \
           --format='{{.Size}}' | awk '{printf "%.1f GB", $1/1024/1024/1024}')
    echo "      hl-unification:humble — OK (${SIZE})"
else
    echo "[ERRO] Imagem não encontrada após o build."
    exit 1
fi

# ── Concluído ─────────────────────────────────────────────
echo ""
echo "================================================"
echo " Instalação concluída!"
echo "================================================"
echo ""
echo "Abrir shell de desenvolvimento:"
echo "  docker compose -f docker/docker-compose.yml run --rm dev"
echo ""
echo "Rodar o deploy (t1_walk.pt):"
echo "  docker compose -f docker/docker-compose.yml up deploy"
echo ""
echo "Rodar o brain (hsl-player):"
echo "  docker compose -f docker/docker-compose.yml up brain"
echo ""
echo "Rodar brain + deploy juntos:"
echo "  docker compose -f docker/docker-compose.yml up brain deploy"
echo ""
