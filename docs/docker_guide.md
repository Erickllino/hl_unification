# Guia Docker — HL Unification

Ambiente de desenvolvimento com Ubuntu 22.04 + ROS2 Humble dentro do Docker,
compatível com o SDK da Booster. O simulador roda nativamente no host; o brain
e o deploy rodam no container. Os tópicos ROS2 fluem entre os dois via
`network_mode: host`.

---

## Instalação

### 1. Pré-requisitos

```bash
# Instalar Docker (se não tiver)
bash install_docker.sh

# Após instalar, aplicar grupo docker sem reiniciar
newgrp docker

# Permitir X11 do container (para visualizadores)
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

## Tópicos que o simulador deve publicar

Para que o brain funcione em simulação, o `hl_sim` deve publicar:

| Tópico | Tipo | Descrição |
|--------|------|-----------|
| `/booster_vision/detection` | `vision_interface/msg/Detections` | Bola, postes, marcadores, oponentes |
| `/booster_vision/line_segments` | `vision_interface/msg/LineSegments` | Linhas do campo |
| `/odometer_state` | `booster_interface/msg/Odometer` | Odometria (posição/velocidade) |
| `/low_state` | `booster_interface/msg/LowState` | Estado das juntas |
| `/head_pose` | `geometry_msgs/msg/Pose` | Orientação da cabeça |
| `/robocup/game_controller` | `game_controller_interface/msg/GameControlData` | Estado do jogo |

O brain publica comandos em:

| Tópico | Tipo | Descrição |
|--------|------|-----------|
| `/api_req_msg` | `booster_msgs/msg/RpcReqMsg` | Comandos de movimento (kMove, etc.) |
| `/rl_move` | `geometry_msgs/msg/Twist` | Velocidade para o deploy RL |

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
│   ├── docker-compose.yml      # serviços: dev, brain, deploy
│   └── entrypoint.sh           # source ROS2 + workspace no startup
├── booster_internal_headers/   # headers proprietários (copiar do robô)
├── hsl-player/
│   └── src/brain/
│       ├── config/config.yaml  # player_id, role, parâmetros do jogo
│       └── launch/launch.py    # aceita player_id:=N, role:=X
├── install_docker.sh           # instala Docker + builda a imagem
└── docs/
    └── docker_guide.md         # este arquivo
```
