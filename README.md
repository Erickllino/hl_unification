# HL Unification — Booster T1

Monorepo unificado do robô humanoide **Booster T1** para RoboCup, integrando:

| Módulo | Função |
|--------|--------|
| `hsl-player` | Lógica de jogo — behaviour trees, visão (YOLO/TensorRT), game controller (ROS2) |
| `booster_deploy` | Políticas de RL — locomotion, simulação MuJoCo e deploy no robô real |
| `booster_assets` | Modelos do robô (URDF/MJCF) e motion data |
| `booster_robotics_sdk` | SDK C++ com bindings Python para interface com hardware |

> Para documentação detalhada de cada módulo, consulte [docs/](docs/).

---

## Instalação

### No PC (simulação)

```bash
bash install_sim.sh
```

Instala o `uv` e cria o ambiente Python com todas as dependências para rodar a simulação MuJoCo localmente.

### No robô (jogo + deploy real)

```bash
bash install_robot.sh
```

Compila o SDK C++, instala as dependências Python e compila o `hsl-player` com colcon.

**Pré-requisito:** ROS2 Humble instalado (`sudo apt install ros-humble-desktop`).

---

## 1. Rodar o jogo (hsl-player)

O `hsl-player` controla toda a lógica de jogo: behaviour trees, visão e comunicação com o árbitro.

### Build (primeira vez ou após mudanças no código)

```bash
cd hsl-player
colcon build --symlink-install
source install/setup.bash
```

### Iniciar no jogo real

```bash
cd hsl-player
./scripts/start.sh
```

### Parar

```bash
./scripts/stop.sh
```

### Simulação (rcsssmj)

```bash
cd hsl-player
./scripts/sim_start.sh

# Para parar:
./scripts/sim_stop.sh
```

### Pacotes ROS2

| Pacote | Tipo | Função |
|--------|------|--------|
| `brain` | C++ | Behaviour trees — lógica de jogo |
| `vision` | C++ | Detecção de bola/jogadores (YOLO + TensorRT) |
| `game_controller` | C++ | Recebe comandos do árbitro via UDP |
| `sound_play` | Python | Reprodução de sons |
| `booster_msgs` | ROS2 msgs | Mensagens customizadas |
| `booster_ros2_interface` | C++ | Interface com o SDK do robô |

---

## 2. Rodar o robô via deploy (RL direto)

O `booster_deploy` roda uma política RL (ex: `t1_walk.pt`) diretamente no hardware, sem a camada de jogo do `hsl-player`.

**Pré-requisitos:**
- Firmware Booster >= v1.4 na **motion board** do T1
- SDK compilado no robô (feito pelo `install_robot.sh`)

### Iniciar

```bash
source .venv/bin/activate
source /opt/booster/BoosterRos2Interface/install/setup.bash
python3 booster_deploy/scripts/deploy.py --task t1_walk
```

### Sequência de ativação no robô

1. Pressionar **X** no joystick (ou `x` no teclado) → entra em modo `kCustom`
2. Pressionar **A** no joystick (ou `r` no teclado) → inicia a política `t1_walk.pt`
3. O robô passa a responder aos comandos de velocidade

### Controles (teclado)

| Tecla | Ação |
|-------|------|
| `w / s` | vx — frente / trás |
| `a / d` | vy — esquerda / direita |
| `q / e` | vyaw — rotação esq/dir |
| `Espaço` | parar |

### Listar tasks disponíveis

```bash
python3 booster_deploy/scripts/deploy.py --list
```

---

## 3. Rodar a simulação (MuJoCo)

Testa a política RL localmente sem o robô físico.

```bash
source .venv/bin/activate
python3 booster_deploy/scripts/deploy.py --task t1_walk --mujoco
```

O viewer do MuJoCo abrirá com o T1. Use os mesmos controles de teclado da seção anterior.

### Limites conhecidos do `t1_walk.pt`

| Eixo | Mínimo útil | Máximo seguro | Observação |
|------|-------------|---------------|------------|
| vx   | 0.4 m/s     | ~2.0 m/s      | > 1.35 deriva para esquerda |
| vy   | 0.4 m/s     | ~1.0 m/s      | > 1.0 instável |
| vyaw | 0.25 rad/s  | ~2.0 rad/s    | > 2.0 instável |

> Convenção: vx+ = frente, vy+ = esquerda, vyaw+ = anti-horário.

### Teste de limites automático

```bash
source .venv/bin/activate
python3 booster_deploy/inject_values.py
```

Incrementa automaticamente `vx`, `vy` ou `vyaw` a cada 10s para mapear os limites do modelo.

---

## 4. SDK (booster_robotics_sdk)

Necessário apenas no **robô real** para que o `booster_deploy` acesse o hardware.
O `install_robot.sh` já compila e instala automaticamente.

### Compilação manual

```bash
cd booster_robotics_sdk
sudo ./install.sh                       # dependências do sistema
mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j$(nproc)
sudo make install
```

### Verificar instalação

```bash
python3 -c "import booster_robotics_sdk_python; print('SDK OK')"
```

---

## Estrutura do repositório

```
hl_unification/
├── booster_assets/            # Modelos do robô (URDF/MJCF) e motion data
├── booster_deploy/            # Políticas RL + controllers (MuJoCo e robô real)
├── booster_robotics_sdk/      # SDK C++ + bindings Python (interface hardware)
├── hsl-player/                # ROS2 — behaviour trees, visão, game controller
├── docs/                      # Documentação detalhada de cada módulo
│   ├── booster_deploy.md
│   ├── booster_assets.md
│   ├── booster_robotics_sdk.md
│   ├── hsl_player.md
│   └── dev_guide.md
├── install_sim.sh             # Instalação para PC (simulação)
├── install_robot.sh           # Instalação para o robô
├── pyproject.toml             # UV workspace Python
└── .python-version            # Python 3.10
```

---

## Notas

- **booster_robotics_sdk** está excluído do UV workspace porque requer compilação C++. No robô, use `install_robot.sh`.
- **hsl-player** é compilado com colcon (ROS2) e não faz parte do `.venv` Python.
- **CUDA**: se o driver NVIDIA for antigo, o torch roda em CPU. Para GPU, instale uma versão de torch compatível com o driver.
- Para integração `hsl-player` ↔ `booster_deploy` via ROS2 (`/booster/cmd_vel`), consulte [BACKLOG.md](BACKLOG.md).
