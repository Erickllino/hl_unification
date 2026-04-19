# hsl-player — Lógica de Jogo RoboCup

O `hsl-player` é o stack ROS2 do Booster T1 para RoboCup. Gerencia a estratégia de jogo via behaviour trees, detecção visual (YOLO + TensorRT) e comunicação com o árbitro eletrônico.

---

## Pacotes ROS2

| Pacote | Linguagem | Função |
|--------|-----------|--------|
| `brain` | C++ | Behaviour trees — lógica de jogo principal |
| `vision` | C++ | Detecção de bola/jogadores/campo (YOLO + TensorRT) |
| `game_controller` | C++ | Recebe estado da partida do árbitro via UDP |
| `sound_play` | Python | Reprodução de sons |
| `booster_msgs` | — | Mensagens ROS2 customizadas do projeto |
| `booster_ros2_interface` | C++ | Interface com o SDK do robô (hardware) |

---

## Build

```bash
cd hsl-player
colcon build --symlink-install
source install/setup.bash
```

Para compilar sem o pacote `vision` (sem GPU/CUDA — útil em simulação ou PC sem placa Nvidia):

```bash
colcon build --symlink-install --packages-skip vision \
    --cmake-args -DCMAKE_BUILD_TYPE=Release \
    --parallel-workers $(nproc)
source install/setup.bash
```

---

## Iniciar

### Jogo real

```bash
cd hsl-player
./scripts/start.sh
```

### Simulação (rcsssmj)

```bash
cd hsl-player
./scripts/sim_start.sh
```

### Parar

```bash
./scripts/stop.sh      # jogo real
./scripts/sim_stop.sh  # simulação
```

### Launch manual (com parâmetros)

```bash
ros2 launch brain launch.py tree:=game.xml player_id:=1 role:=striker
```

| Parâmetro | Padrão | Descrição |
|-----------|--------|-----------|
| `tree` | `game.xml` | Arquivo do behaviour tree (`config/behavior_trees/`) |
| `player_id` | config.yaml | ID do jogador (1–5) |
| `role` | config.yaml | `striker` ou `goal_keeper` |
| `sim` | `false` | Ativa `use_sim_time` (para simulação com Docker) |
| `disable_log` | `false` | Desativa log em arquivo |
| `disable_com` | `false` | Desativa comunicação entre robôs |

---

## Estrutura de Diretórios

```
hsl-player/
└── src/
    └── brain/
        ├── config/
        │   ├── behavior_trees/     # XMLs dos behaviour trees
        │   │   ├── game.xml        # tree principal de jogo
        │   │   └── test.xml        # tree para testes
        │   └── config.yaml         # parâmetros: player_id, role, IPs, etc.
        ├── include/                # headers C++
        ├── launch/
        │   └── launch.py           # arquivo de launch ROS2
        └── src/                    # código-fonte C++
```

---

## Behaviour Trees

O `brain` usa a biblioteca [BehaviorTree.CPP](https://www.behaviortree.dev/) para definir a estratégia de jogo. Os trees são arquivos XML em `config/behavior_trees/`.

Os XMLs definem a lógica de decisão (atacar, defender, posicionar, chutar), trocando de comportamento em resposta ao estado do jogo recebido pelo `game_controller` via `/robocup/game_controller`.

---

## Tópicos ROS2

Para a referência completa de tópicos, veja [ros2_topics.md](ros2_topics.md).

**Resumo:**

| Direção | Tópicos principais |
|---------|-------------------|
| Publica | `LocoApiTopicReq` (comandos hardware), `/rl_move` (velocidade → t1_walk), `/play_sound`, `/speak` |
| Assina | `/booster_vision/detection`, `/low_state`, `/robocup/game_controller`, `/odometer_state`, `/remote_controller_state` |

---

## Integração com booster_deploy (RL)

O brain publica comandos de velocidade em `/rl_move` (`geometry_msgs/Twist`), consumidos pelo `booster_deploy` para alimentar a política `t1_walk.pt`.

```
brain_node → /rl_move (vx, vy, vyaw) → deploy (t1_walk.pt) → /joint_ctrl → hardware/sim
```

O robô deve estar em modo `kCustom` para que o `booster_deploy` controle as juntas diretamente.
Ver [dev_guide.md](dev_guide.md) para detalhes sobre o deploy RL.

---

## Desenvolvimento via Docker

Para compilar e rodar o hsl-player dentro do Docker (recomendado para desenvolvimento no PC), veja [docker_guide.md](docker_guide.md).
