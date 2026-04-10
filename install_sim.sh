#!/usr/bin/env bash
# install_sim.sh — Instalação para PC (simulação MuJoCo)
# Instala uv e as dependências Python do workspace (booster_deploy + booster_assets + torch + mujoco)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "================================================"
echo " HL Unification — Instalação para Simulação"
echo "================================================"
echo ""

# --- 1. Verificar Python 3.10 ---
echo "[1/3] Verificando Python 3.10..."
if ! python3.10 --version &>/dev/null; then
    echo "[ERRO] Python 3.10 não encontrado."
    echo "       Instale com: sudo apt install python3.10 python3.10-venv"
    exit 1
fi
echo "      $(python3.10 --version) — OK"

# --- 2. Instalar uv ---
echo ""
echo "[2/3] Verificando uv..."
if ! command -v uv &>/dev/null; then
    echo "      uv não encontrado. Instalando..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    # Adiciona ao PATH da sessão atual
    export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
    if ! command -v uv &>/dev/null; then
        echo "[ERRO] Falha ao instalar uv. Tente manualmente:"
        echo "       curl -LsSf https://astral.sh/uv/install.sh | sh"
        echo "       Em seguida, reinicie o terminal e rode este script novamente."
        exit 1
    fi
fi
echo "      $(uv --version) — OK"

# --- 3. Criar venv e instalar dependências ---
echo ""
echo "[3/3] Criando ambiente virtual e instalando dependências..."
echo "      (booster_assets, booster_deploy, torch, mujoco, scipy, numpy, evdev)"
uv sync

echo ""
echo "================================================"
echo " Instalação concluída!"
echo "================================================"
echo ""
echo "Ativar o ambiente:"
echo "  source .venv/bin/activate"
echo ""
echo "Rodar a simulação MuJoCo:"
echo "  python3 booster_deploy/scripts/deploy.py --task t1_walk --mujoco"
echo ""
echo "Listar tasks disponíveis:"
echo "  python3 booster_deploy/scripts/deploy.py --list"
echo ""
