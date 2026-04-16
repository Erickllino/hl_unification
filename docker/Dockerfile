# ============================================================
#  HL Unification — Imagem de desenvolvimento
#  Base: Ubuntu 22.04 + ROS2 Humble
#
#  Contém:
#   - ROS2 Humble + deps do brain (C++)
#   - Booster Robotics SDK (C++ libs + Python bindings)
#   - Rerun SDK (C++ visualização/logging)
#   - Python 3.10 + uv + booster_deploy + booster_assets
#
#  NOTA: booster_internal (API interna do robô) NÃO está
#  incluído aqui pois é proprietário. O brain compila sem
#  ele apenas se os headers estiverem disponíveis no host
#  e montados via volume (ver docker-compose.yml).
# ============================================================

FROM osrf/ros:humble-desktop

ARG RERUN_VERSION=0.21.0
ARG DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

# ── 1. Dependências do sistema ────────────────────────────
RUN apt-get update && apt-get install -y \
    # build
    build-essential cmake git curl wget unzip \
    # SDK Booster
    libssl-dev libasio-dev libtinyxml2-dev \
    # brain C++
    libeigen3-dev libopencv-dev libyaml-cpp-dev \
    # Python 3.10
    python3.10 python3.10-venv python3.10-dev python3-pip \
    # ROS2 extras
    ros-humble-behaviortree-cpp \
    ros-humble-backward-ros \
    ros-humble-tf2 \
    ros-humble-tf2-ros \
    ros-humble-tf2-geometry-msgs \
    ros-humble-cv-bridge \
    python3-colcon-common-extensions \
    python3-rosdep \
    # utilitários
    lsb-release sudo \
    && rm -rf /var/lib/apt/lists/*

# ── 2. Rerun SDK (C++) ────────────────────────────────────
# O zip contém o código-fonte do SDK (não pré-compilado).
# Compilamos e instalamos em /usr/local para que find_package()
# encontre rerun_sdkConfig.cmake automaticamente.
RUN wget -q "https://github.com/rerun-io/rerun/releases/download/${RERUN_VERSION}/rerun_cpp_sdk.zip" \
         -O /tmp/rerun_cpp_sdk.zip && \
    unzip -q /tmp/rerun_cpp_sdk.zip -d /tmp/rerun_src && \
    cmake -B /tmp/rerun_build -S /tmp/rerun_src/rerun_cpp_sdk \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX=/usr/local && \
    cmake --build /tmp/rerun_build -j"$(nproc)" && \
    cmake --install /tmp/rerun_build && \
    rm -rf /tmp/rerun_src /tmp/rerun_build /tmp/rerun_cpp_sdk.zip && \
    ldconfig

# ── 3. Booster Robotics SDK (C++ + Python bindings) ──────
# Copia o SDK do contexto de build e instala os headers/libs
COPY booster_robotics_sdk/ /opt/booster_robotics_sdk/

RUN apt-get update && \
    cd /opt/booster_robotics_sdk && \
    bash install.sh && \
    rm -rf /var/lib/apt/lists/* && \
    # Compila Python bindings
    pip install scikit-build-core pybind11 pybind11-stubgen && \
    mkdir -p build && cd build && \
    cmake .. -DBUILD_PYTHON_BINDING=ON && \
    make -j"$(nproc)" && \
    make install && \
    ldconfig

# ── 4. uv (gerenciador de pacotes Python) ────────────────
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:/root/.cargo/bin:${PATH}"

# ── 5. Workspace Python (booster_deploy + booster_assets) ─
WORKDIR /workspace

# Copia arquivos de configuração do workspace Python primeiro
# (para aproveitar cache do Docker se só o código mudar)
COPY pyproject.toml uv.lock ./
COPY booster_assets/ ./booster_assets/
COPY booster_deploy/ ./booster_deploy/

RUN uv sync --frozen

# ── 6. Workspace ROS2 (hsl-player) ───────────────────────
COPY hsl-player/src/ /workspace/hsl-player/src/

# Inicializa rosdep e instala dependências dos pacotes ROS2
RUN rosdep update && \
    rosdep install --from-paths /workspace/hsl-player/src \
        --ignore-src -r -y \
        --skip-keys "booster_interface booster_msgs vision_interface game_controller_interface rerun_sdk" \
    || true

# Compila o workspace ROS2
# Se booster_internal não estiver disponível, o brain falhará
# mas os outros pacotes (msgs, interfaces) serão compilados.
RUN source /opt/ros/humble/setup.bash && \
    cd /workspace/hsl-player && \
    colcon build --symlink-install \
        --cmake-args -DCMAKE_BUILD_TYPE=Release \
        --parallel-workers "$(nproc)" \
    || echo "[AVISO] Alguns pacotes falharam ao compilar (ex: brain sem booster_internal)"

# ── 7. Entrypoint ─────────────────────────────────────────
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

WORKDIR /workspace
ENTRYPOINT ["/entrypoint.sh"]
CMD ["bash"]
