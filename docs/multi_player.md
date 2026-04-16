# Rodando Múltiplos Jogadores — Docker

Cada jogador é um container separado rodando um `brain_node` com um
`player_id` diferente. Como todos usam `network_mode: host`, os brains
se enxergam automaticamente via ROS2 DDS.

---

## Pré-requisito

Compile o hsl-player uma vez (a compilação fica no volume compartilhado):

```bash
docker compose -f docker/docker-compose.yml run --rm dev bash -c "
    cd /workspace/hsl-player &&
    colcon build --symlink-install --packages-skip vision \
        --cmake-args -DCMAKE_BUILD_TYPE=Release \
        --parallel-workers \$(nproc)
"
```

---

## Rodar cada jogador

Abra um terminal por jogador. Cada um recebe um `player_id` e um `role`:

**Jogador 1 — Goleiro:**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p1 dev bash -c "
        source /workspace/hsl-player/install/setup.bash &&
        ros2 launch brain launch.py \
            tree:=game.xml \
            player_id:=1 \
            role:=goal_keeper
    "
```

**Jogador 2 — Atacante:**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p2 dev bash -c "
        source /workspace/hsl-player/install/setup.bash &&
        ros2 launch brain launch.py \
            tree:=game.xml \
            player_id:=2 \
            role:=striker
    "
```

**Jogador 3 — Atacante:**
```bash
docker compose -f docker/docker-compose.yml run --rm \
    --name hl_brain_p3 dev bash -c "
        source /workspace/hsl-player/install/setup.bash &&
        ros2 launch brain launch.py \
            tree:=game.xml \
            player_id:=3 \
            role:=striker
    "
```

### Parâmetros disponíveis no launch

| Parâmetro | Valores | Descrição |
|-----------|---------|-----------|
| `player_id` | `1`–`5` | ID único do jogador |
| `role` | `striker`, `goal_keeper` | Papel em campo |
| `tree` | `game.xml`, etc. | Arquivo do behavior tree |
| `sim` | `true` / `false` | Ativa `use_sim_time` (usar com simulador) |
| `disable_log` | `true` / `false` | Desativa gravação de log em arquivo |
| `disable_com` | `true` / `false` | Desativa comunicação entre jogadores |

---

## Verificar que os brains se enxergam

Em qualquer terminal (dentro ou fora do Docker):

```bash
# Dentro de um container:
ros2 node list
# Deve mostrar /brain_node de cada jogador

ros2 topic echo /robocup/game_controller
# Todos os brains publicam e assinam esse tópico
```

---

## Abrir terminal extra em container rodando

```bash
# Ver containers ativos
docker ps

# Entrar no container do jogador 2
docker exec -it hl_brain_p2 bash
```

---

## Rodar com simulador (hl_sim)

Passe `sim:=true` e `disable_log:=true` para cada brain:

```bash
ros2 launch brain launch.py \
    tree:=game.xml \
    player_id:=1 \
    role:=goal_keeper \
    sim:=true \
    disable_log:=true
```

O simulador (`hl_sim`) roda nativamente no host. Como todos usam
`network_mode: host`, os tópicos do sim aparecem automaticamente
para todos os brains.

---

## Acesso aos tópicos ROS2 fora do Docker

Com `network_mode: host`, os tópicos dos containers são visíveis no host
diretamente — sem configuração extra.

```bash
# No host:
source /opt/ros/humble/setup.bash  # Ubuntu 22.04
# ou
source /opt/ros/jazzy/setup.bash   # Ubuntu 24.04

export ROS_DOMAIN_ID=0  # deve ser igual ao docker-compose.yml

ros2 topic list
ros2 node list
ros2 topic echo /low_state
```

> ROS2 Jazzy (Ubuntu 24.04) e Humble (Ubuntu 22.04 / Docker) são compatíveis
> via FastDDS desde que `ROS_DOMAIN_ID` seja igual nos dois lados.

---

## Estrutura de player_id × role

Configure de acordo com as regras da partida:

| player_id | role sugerido | Notas |
|-----------|--------------|-------|
| 1 | `goal_keeper` | Goleiro — fica na área |
| 2 | `striker` | Atacante principal |
| 3 | `striker` | Atacante secundário |
| 4 | `striker` | — |
| 5 | `striker` | — |

Os valores padrão ficam em `hsl-player/src/brain/config/config.yaml`.
O argumento de launch sobrescreve apenas para aquela execução.
