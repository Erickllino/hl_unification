# Guia Docker — HL Unification

Ambiente de desenvolvimento com Ubuntu 22.04 + ROS2 Humble dentro do Docker,
compatível com o SDK da Booster. Brain e deploy rodam no container `brain`;
o simulador MuJoCo roda no container `sim`. Os tópicos ROS2 fluem entre os dois via
`network_mode: host`.

---

## Instalação

### 1. Pré-requisitos

```bash
# Instalar Docker (se não tiver)
bash install_docker.sh

# Após instalar, aplicar grupo docker sem reiniciar
newgrp docker

# Permitir X11 do container (para visualizadores — MuJoCo viewer, RViz2, etc.)
# IMPORTANTE: rodar isso toda vez que abrir uma nova sessão
xhost +local:docker
```

### 2. Copiar headers proprietários do robô

Os headers `booster_internal` ficam no robô real e são necessários para compilar
o brain com todas as funcionalidades (chutes RL, agachamentos, etc.).

```bash
scp -r booster@<IP_DO_ROBO>:/usr/local/include/booster_internal \
    ~/Documents/hl_unification/booster_internal_headers
```

### 3. Build da imagem

```bash
cd ~/Documents/hl_unification
docker compose -f docker/docker-compose.yml build
```

A primeira vez leva ~15-20 minutos (compila Rerun SDK + Arrow).
As builds seguintes são rápidas por causa do cache.

---

## Compilar o hsl-player

Abra um shell de desenvolvimento:

```bash
docker compose -f docker/docker-compose.yml run --rm dev
```

Dentro do container, compile (pulando o pacote `vision` que requer CUDA):

```bash
cd /workspace/hsl-player
colcon build --symlink-install --packages-skip vision \
    --cmake-args -DCMAKE_BUILD_TYPE=Release \
    --parallel-workers $(nproc)
source install/setup.bash
```

> A compilação leva ~20-30 segundos na segunda vez (incremential).
> Mudanças nos arquivos do host aparecem instantaneamente no container — não é
> necessário rebuildar a imagem, só recompilar com colcon.

---

## Rodar um brain

```bash
# Dentro do container (após source install/setup.bash)
ros2 launch brain launch.py tree:=game.xml
```

Parâmetros disponíveis:

| Parâmetro | Padrão | Descrição |
|-----------|--------|-----------|
| `tree` | `game.xml` | Arquivo do behavior tree |
| `player_id` | (do config.yaml) | ID do jogador (1–5) |
| `role` | (do config.yaml) | `striker` ou `goal_keeper` |
| `sim` | `false` | Ativa `use_sim_time` |
| `disable_log` | `false` | Desativa log em arquivo |
| `disable_com` | `false` | Desativa comunicação entre robôs |

---

## Rodar múltiplos jogadores

Cada jogador roda em um container separado. Abra um terminal por jogador:

**Terminal 1 — Jogador 1 (striker):**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p1 dev bash -c "
        cd /workspace/hsl-player &&
        source install/setup.bash &&
        ros2 launch brain launch.py tree:=game.xml player_id:=1 role:=striker
    "
```

**Terminal 2 — Jogador 2 (goal_keeper):**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p2 dev bash -c "
        cd /workspace/hsl-player &&
        source install/setup.bash &&
        ros2 launch brain launch.py tree:=game.xml player_id:=2 role:=goal_keeper
    "
```

**Terminal 3 — Jogador 3 (striker):**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p3 dev bash -c "
        cd /workspace/hsl-player &&
        source install/setup.bash &&
        ros2 launch brain launch.py tree:=game.xml player_id:=3 role:=striker
    "
```

> Todos os containers usam `network_mode: host`, então os brains se enxergam
> pelo ROS2 DDS automaticamente (mesmos tópicos, mesmo `ROS_DOMAIN_ID=0`).

---

## Acessar tópicos ROS2 fora do Docker

Como todos os containers usam `network_mode: host`, os tópicos publicados dentro
do Docker são **diretamente visíveis no host** — sem port forwarding, sem
configuração extra.

No host (com ROS2 sourced):

```bash
source /opt/ros/humble/setup.bash   # ou jazzy se for Ubuntu 24.04
export ROS_DOMAIN_ID=0              # deve ser igual ao do docker-compose.yml

# Listar todos os tópicos (incluindo os do Docker)
ros2 topic list

# Ver o que o brain está publicando
ros2 topic echo /booster_vision/detection

# Ver estado do jogo
ros2 topic echo /robocup/game_controller
```

> **Ubuntu 24.04 + ROS2 Jazzy**: o DDS é compatível com Humble via FastDDS.
> Os tópicos aparecem normalmente desde que `ROS_DOMAIN_ID` seja igual.

---

## Simulação via Docker

### Arquitetura

```
┌─────────────────────────────────────────┐     ┌──────────────────────────────┐
│  Container: brain                        │     │  Container: sim              │
│                                          │     │                              │
│  brain_node  ──► /rl_move (Twist)        │     │  hl_sim_bridge               │
│                       │                  │     │   ├─ ros2_agent (T1_1)       │
│  deploy (t1_walk.pt)  │                  │     │   └─ standalone  (T1_2)      │
│   └─ rede neural  ◄───┘                  │     │                              │
│   └─ /joint_ctrl (LowCmd) ──────────────────► MuJoCo physics                │
│                                          │     │                              │
│                          ◄──────────────────── /low_state (21 juntas + IMU)  │
│                          ◄──────────────────── /booster_vision/detection     │
└─────────────────────────────────────────┘     └──────────────────────────────┘
                           network_mode: host (DDS compartilhado)
```

**Fluxo de dados:**

1. `brain_node` publica `/rl_move` (Twist com vx/vy/vyaw)
2. `deploy --sim` recebe `/rl_move`, roda inferência na rede neural `t1_walk.pt`
3. `deploy` publica `/joint_ctrl` (LowCmd com 21 posições de junta)
4. `ros2_agent` no `hl_sim` recebe `/joint_ctrl` e escreve em `mj_data.ctrl`
5. MuJoCo avança a física e o robô se move no viewer
6. `hl_sim` publica `/low_state` (juntas + IMU) e detecções de volta ao brain

**Flag `--sim` no deploy:**

O `booster_deploy` com `--sim` pula a inicialização do SDK de hardware (`B1LocoClient`,
`ChannelFactory`) e roda apenas a inferência da rede neural + publicação ROS2.
Nenhum SDK Booster é necessário para simulação.

### Pré-requisito: repositório `hl_sim`

O simulador MuJoCo deve estar em:

```
~/Documents/
├── hl_unification/   ← este repo
└── hl_sim/           ← simulador (montado pelo docker-compose)
```

### Iniciar a simulação 1v1

```bash
# Permitir X11 para o viewer MuJoCo (rodar a cada sessão nova)
xhost +local:docker

cd ~/Documents/hl_unification
docker compose -f docker/docker-compose.yml up sim brain
```

O viewer MuJoCo abrirá com dois robôs T1:
- **T1_1** (branco): controlado pelo brain via `/joint_ctrl`
- **T1_2** (vermelho): agente scripted independente

### Recompilar o brain após mudanças no código C++

Qualquer mudança em `hsl-player/src/brain/src/*.cpp` requer recompilação:

```bash
docker compose -f docker/docker-compose.yml run --rm dev bash -c "
  cd /workspace/hsl-player &&
  colcon build --symlink-install --packages-select brain \
    --cmake-args -DCMAKE_BUILD_TYPE=Release
"
```

> **Atenção**: se o brain não publicar `/rl_move`, verificar se o binário está
> atualizado. O timestamp de `build/brain/CMakeFiles/brain_node.dir/src/robot_client.cpp.o`
> deve ser mais recente que `src/brain/src/robot_client.cpp`.

### Verificar tópicos em tempo real

```bash
# Em outro terminal, entrar no container brain
docker exec -it hl_brain bash

# Ver se brain publica /rl_move
source /workspace/hsl-player/install/setup.bash
ros2 topic echo /rl_move

# Ver se deploy publica /joint_ctrl
ros2 topic hz /joint_ctrl

# Ver nós ativos
ros2 node list
# Esperado: /brain_node, /brain_node_ext, /hl_sim_bridge
```

### Topologia de nós ROS2

| Nó | Container | Função |
|----|-----------|--------|
| `/brain_node` | brain | Behaviour trees, visão, game controller |
| `/brain_node_ext` | brain | Extensões do brain (chutes, etc.) |
| `/hl_sim_bridge` | sim | Bridge MuJoCo ↔ ROS2 (único nó do simulador) |

### Multi-agente (futuro)

Para múltiplos brains controlando robôs diferentes, cada brain precisará de um
namespace ROS2 para separar os tópicos:

```bash
# Brain do jogador 1
ros2 launch brain launch.py --ros-args \
    -r /low_state:=/robot1/low_state \
    -r /rl_move:=/robot1/rl_move \
    -r /joint_ctrl:=/robot1/joint_ctrl

# Brain do jogador 2
ros2 launch brain launch.py --ros-args \
    -r /low_state:=/robot2/low_state \
    -r /rl_move:=/robot2/rl_move \
    -r /joint_ctrl:=/robot2/joint_ctrl
```

O `hl_sim` precisará de um `ROS2Agent` por namespace, cada um escrevendo em
`mj_data.ctrl[i*23:(i+1)*23]` para o robô correspondente.

---

## Tópicos do pipeline de simulação

O brain consome:

| Tópico | Tipo | Publicado por |
|--------|------|---------------|
| `/low_state` | `booster_interface/msg/LowState` | `hl_sim` (juntas + IMU) |
| `/booster_vision/detection` | `vision_interface/msg/Detections` | `hl_sim` (bola, robôs) |
| `/odometer_state` | `booster_interface/msg/Odometer` | `hl_sim` (posição) |

O brain publica:

| Tópico | Tipo | Consumido por |
|--------|------|---------------|
| `/rl_move` | `geometry_msgs/msg/Twist` | `deploy --sim` |
| `/joint_ctrl` | `booster_interface/msg/LowCmd` | `hl_sim` (juntas MuJoCo) |

---

## Abrir segundo terminal em container já rodando

```bash
# Ver containers ativos
docker ps

# Entrar no container do jogador 1
docker exec -it hl_brain_p1 bash
```

---

## Estrutura de arquivos relevantes

```
hl_unification/
├── docker/
│   ├── Dockerfile              # imagem Ubuntu 22.04 + ROS2 Humble
│   ├── docker-compose.yml      # serviços: brain (brain+deploy), sim, dev
│   └── entrypoint.sh           # source ROS2 + workspace no startup
├── booster_internal_headers/   # headers proprietários (copiar do robô)
├── booster_deploy/
│   └── scripts/deploy.py       # aceita --sim (sem SDK hardware)
├── hsl-player/
│   └── src/brain/
│       ├── config/config.yaml  # player_id, role, parâmetros do jogo
│       └── launch/launch.py    # aceita player_id:=N, role:=X, deploy:=true/false
├── install_docker.sh           # instala Docker + builda a imagem
└── docs/
    └── docker_guide.md         # este arquivo

# Repositório externo (deve estar em ../hl_sim)
hl_sim/
├── bridge/
│   ├── sim_bridge.py           # loop MuJoCo, _apply() escreve mj_data.ctrl
│   ├── ros2_agent.py           # assina /joint_ctrl, publica /low_state
│   └── agent_interface.py      # SensorState, ActionCmd (joint_pos, sensordata)
├── scenes/soccer_scene.xml     # cena com T1_1 e T1_2
└── scripts/run_ros2.py         # entry-point: ROS2Agent + StandaloneAgent
```
