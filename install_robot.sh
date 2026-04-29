#!/usr/bin/env bash
# install_robot.sh — Instalação para o robô Booster T1
# Compila o SDK C++, instala dependências Python e compila o hsl-player (ROS2/colcon)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "================================================"
echo " HL Unification — Instalação para Robô Real"
echo "================================================"
echo ""

# --- 1. Dependências do sistema ---
echo "[1/5] Instalando dependências do sistema..."
sudo apt-get update -q
sudo apt-get install -y \
    build-essential \
    cmake \
    git \
    libssl-dev \
    libasio-dev \
    libtinyxml2-dev
echo "      Dependências do sistema — OK"

# --- 2. Booster Robotics SDK (C++ + Python bindings) ---
echo ""
echo "[2/5] Compilando booster_robotics_sdk..."
SDK_DIR="$SCRIPT_DIR/booster_robotics_sdk"

if [ ! -d "$SDK_DIR" ]; then
    echo "[ERRO] Pasta booster_robotics_sdk não encontrada em $SDK_DIR"
    exit 1
fi

cd "$SDK_DIR"
echo "      Instalando dependências nativas do SDK..."
sudo ./install.sh

echo "      Compilando SDK e Python bindings..."
mkdir -p build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j"$(nproc)"
sudo make install
cd "$SCRIPT_DIR"
echo "      SDK compilado — OK"

# --- 3. Ambiente Python (uv + workspace) ---
echo ""
echo "[3/5] Instalando ambiente Python com uv..."
if ! command -v uv &>/dev/null; then
    echo "      uv não encontrado. Instalando..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
    if ! command -v uv &>/dev/null; then
        echo "[ERRO] Falha ao instalar uv. Tente manualmente:"
        echo "       curl -LsSf https://astral.sh/uv/install.sh | sh"
        exit 1
    fi
fi
echo "      $(uv --version) — OK"

# IMPORTANTE: No Jetson (JetPack), o torch precisa ser o da NVIDIA com CUDA,
# não a wheel padrão do PyPI. O torch da NVIDIA é instalado em
# /usr/lib/python3/dist-packages e DEVE ser reaproveitado pelo venv via
# --system-site-packages. Sem isso, `uv sync` tenta instalar o torch do PyPI
# que NÃO funciona no aarch64+CUDA do Jetson.
echo "      Criando .venv com acesso aos pacotes do sistema (torch da NVIDIA)..."
uv venv --system-site-packages --clear .venv

echo "      Sincronizando dependências do workspace..."
uv sync
echo "      Dependências Python — OK"

# Verifica se o torch da NVIDIA está acessível
echo "      Verificando torch (deve ser versão NVIDIA/JetPack)..."
if .venv/bin/python -c "import torch; print(f'        torch={torch.__version__}, cuda={torch.version.cuda}')" 2>/dev/null; then
    echo "      torch — OK"
else
    echo "[AVISO] torch não acessível no .venv."
    echo "        Verifique se o torch da NVIDIA está instalado no sistema:"
    echo "          python3 -c 'import torch; print(torch.__version__)'"
    echo "        Se não estiver, instale via JetPack (apt) ou wheel oficial da NVIDIA."
fi

# --- 4. hsl-player (ROS2 / colcon) ---
echo ""
echo "[4/5] Compilando hsl-player..."
HSL_DIR="$SCRIPT_DIR/hsl-player"

if ! command -v colcon &>/dev/null; then
    echo "[AVISO] colcon não encontrado — hsl-player não foi compilado."
    echo "        Para instalar o ROS2 Humble e colcon:"
    echo "          sudo apt install ros-humble-desktop"
    echo "          source /opt/ros/humble/setup.bash"
    echo "          sudo apt install python3-colcon-common-extensions"
    echo "        Em seguida, rode manualmente:"
    echo "          cd hsl-player && colcon build --symlink-install"
else
    if [ ! -d "$HSL_DIR" ]; then
        echo "[AVISO] Pasta hsl-player não encontrada — pulando."
    else
        cd "$HSL_DIR"
        colcon build --symlink-install --parallel-workers "$(nproc)"
        cd "$SCRIPT_DIR"
        echo "      hsl-player compilado — OK"
    fi
fi

# --- 5. Verificar SDK Python ---
echo ""
echo "[5/5] Verificando SDK Python..."
if .venv/bin/python3 -c "import booster_robotics_sdk_python" 2>/dev/null; then
    echo "      booster_robotics_sdk_python importado com sucesso — OK"
else
    echo "[AVISO] booster_robotics_sdk_python não encontrado no .venv."
    echo "        O SDK foi instalado globalmente (sudo make install)."
    echo "        Para usar no .venv, adicione ao PYTHONPATH:"
    echo "          export PYTHONPATH=/usr/local/lib/python3.10/dist-packages:\$PYTHONPATH"
fi

echo ""
echo "================================================"
echo " Instalação concluída!"
echo "================================================"
echo ""
echo "Para rodar o deploy no robô:"
echo "  source .venv/bin/activate"
echo "  source /opt/booster/BoosterRos2Interface/install/setup.bash"
echo "  uv run deploy --task t1_walk"
echo ""
echo "Para rodar o hsl-player (jogo):"
echo "  cd hsl-player"
echo "  source install/setup.bash"
echo "  ./scripts/start.sh"
echo ""
