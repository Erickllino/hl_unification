# HL Unification — Monorepo Booster T1

Monorepo unificado para o robô humanoide Booster T1, integrando simulação,
deploy de modelos RL, SDK de controle e lógica de jogo (RoboCup).

## Estrutura

```
hl_unification/
├── booster_assets/            # Configurações do robô (joints, motions)
├── booster_deploy/            # Treino/deploy de políticas RL (MuJoCo + robô real)
├── booster_robotics_sdk/      # SDK C++ com bindings Python (interface hardware)
├── hsl-player/                # ROS2 workspace — behaviour trees, visão, game controller
├── pyproject.toml             # UV workspace (unifica os pacotes Python)
└── .python-version            # Python 3.10
```

| Pacote | Descrição | Build |
|---|---|---|
| `booster_assets` | Assets do T1 (modelos MJCF, nomes de joints, motions) | Python puro |
| `booster_deploy` | Controllers MuJoCo/robô real + políticas RL (locomotion, mimic) | Python (torch, mujoco) |
| `booster_robotics_sdk` | Bindings pybind11 do SDK C++ da Booster | C++ (scikit-build) |
| `hsl-player` | Brain (behaviour trees), visão (YOLO/TensorRT), game controller, sound | ROS2 / colcon |

## Requisitos

| Ferramenta | Versão | Notas |
|---|---|---|
| Python | 3.10 | Compatível com robô real e ROS2 Humble |
| [uv](https://docs.astral.sh/uv/) | >= 0.4 | Gerenciador de pacotes |
| ROS 2 Humble | — | Apenas para hsl-player e deploy no robô |
| MuJoCo | >= 3.x | Instalado automaticamente pelo uv |
| CMake + GCC | — | Apenas se precisar compilar o booster_robotics_sdk |

## Instalacao

### 1. Instalar uv

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2. Clonar o repositorio

```bash
git clone <url-do-repo> hl_unification
cd hl_unification
```

### 3. Criar venv e instalar dependencias

```bash
uv sync
```

Isso cria o `.venv/` com Python 3.10 e instala todos os pacotes:
`booster_assets`, `booster_deploy`, `torch`, `mujoco`, `scipy`, `numpy`, `evdev`.

### 4. Ativar o ambiente

```bash
source .venv/bin/activate
```

Ou use `uv run <comando>` sem ativar.

## Simulacao MuJoCo (booster_deploy)

### Listar tasks disponiveis

```bash
python booster_deploy/scripts/deploy.py --list
```

Tasks registradas:
- `t1_walk` — Caminhada do T1
- `k1_mj2` — K1 com motion MJ2
- `k1_fight` — K1 com motion de luta

### Rodar simulacao

```bash
python booster_deploy/scripts/deploy.py --task t1_walk --mujoco
```

O MuJoCo abre o viewer com o T1. Comandos de velocidade via stdin:

```
Set command (x, y, yaw): 0.5 0.0 0.0    # andar para frente
Set command (x, y, yaw): 0.0 0.3 0.0    # andar para o lado
Set command (x, y, yaw): 0.0 0.0 1.0    # girar
```

### Teste de limites automatico

```bash
python booster_deploy/inject_values.py
```

Incrementa automaticamente `vx`, `vy` ou `vyaw` a cada 10s para mapear limites do modelo.

Limites conhecidos do `t1_walk.pt`:

| Comando | Min util | Max seguro | Notas |
|---|---|---|---|
| vx | 0.4 m/s | ~2.0 m/s | > 1.35 deriva para esquerda |
| vy | 0.4 m/s | ~1.0 m/s | > 1.0 instavel |
| vyaw | 0.25 rad/s | ~2.0 rad/s | > 2.0 instavel |

## Deploy no robo real (T1)

### 1. Compilar o SDK no robo

```bash
cd booster_robotics_sdk
mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j$(nproc)
sudo make install
```

### 2. Instalar dependencias no robo

O robo ja tem Python 3.10 e ROS2 Humble:

```bash
pip install torch scipy evdev
pip install -e booster_assets
pip install -e booster_deploy
```

### 3. Iniciar ROS2

```bash
source /opt/booster/BoosterRos2Interface/install/setup.bash
```

### 4. Rodar

```bash
python3 booster_deploy/scripts/deploy.py --task t1_walk
```

Sequencia:
1. Pressionar `x` (ou botao X do joystick) → entra em modo `kCustom`
2. Pressionar `r` (ou botao A do joystick) → inicia o `t1_walk.pt`
3. Robo responde a comandos de velocidade

## hsl-player (ROS2)

O `hsl-player` e um workspace ROS2 separado, compilado com colcon:

```bash
cd hsl-player
colcon build
source install/setup.bash
```

### Iniciar o sistema completo

```bash
# Terminal 1 — brain + vision + game_controller
./scripts/sim_start.sh

# Parar
./scripts/sim_stop.sh
```

### Pacotes ROS2

| Pacote | Tipo | Funcao |
|---|---|---|
| `brain` | C++ | Behaviour trees — logica de jogo |
| `vision` | C++ | Detecao de bola/jogadores (YOLO + TensorRT) |
| `game_controller` | C++ | Recebe comandos do arbitro via UDP |
| `sound_play` | Python | Reproducao de sons |
| `booster_msgs` | ROS2 msgs | Mensagens customizadas |
| `booster_ros2_interface` | C++ | Interface com o SDK do robo |

## Adicionar uma nova task de RL

```
booster_deploy/tasks/minha_task/
├── __init__.py          # register_task("nome", MinhaCfg())
├── task.py              # Policy + ControllerCfg
└── models/
    └── modelo.pt        # checkpoint TorchScript
```

Verificar registro:

```bash
python booster_deploy/scripts/deploy.py --list
```

## Notas

- **booster_robotics_sdk** esta excluido do UV workspace porque precisa compilar C++.
  No robo real, compile separadamente (ver secao Deploy).
- **hsl-player** e compilado com colcon (ROS2), nao faz parte do venv Python.
- **CUDA**: se o driver NVIDIA for antigo, torch funciona em CPU. Para GPU,
  atualize o driver ou instale uma versao de torch compativel.
- Para integracao hsl-player + booster_deploy via ROS2, consulte a documentacao
  em [BACKLOG.md](BACKLOG.md).
