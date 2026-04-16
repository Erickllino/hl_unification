# Docker — HL Unification

Ambiente de desenvolvimento com **Ubuntu 22.04 + ROS2 Humble**, compatível com o SDK da Booster.

O simulador (MuJoCo) e visualizações pesadas rodam **nativamente** no host. O brain e o deploy rodam aqui dentro. Os tópicos ROS2 fluem entre os dois via `network_mode: host`.

---

## Pré-requisitos

```bash
# Instalar Docker
sudo apt install docker.io docker-compose-v2

# Permitir rodar sem sudo (requer logout/login)
sudo usermod -aG docker $USER

# Permitir X11 do container (para serviço dev com visualização)
xhost +local:docker
```

---

## Build da imagem

```bash
cd hl_unification

# Build completo (primeira vez ~10-15 min)
docker compose -f docker/docker-compose.yml build
```

---

## Uso

### Shell de desenvolvimento (mais comum)
```bash
docker compose -f docker/docker-compose.yml run --rm dev
```
Dentro do container, tudo já está configurado:
```bash
# Verificar tópicos ROS2 (inclusive do simulador nativo)
ros2 topic list

# Rodar o deploy
uv run deploy --task t1_walk --auto-start

# Compilar o hsl-player após mudanças
cd /workspace/hsl-player
colcon build --symlink-install --packages-select brain
```

### Rodar brain + deploy juntos
```bash
# Em terminais separados:
docker compose -f docker/docker-compose.yml up brain
docker compose -f docker/docker-compose.yml up deploy
```

### Rodar o simulador nativo (fora do container)
```bash
# No host Ubuntu 24.04 — acessa os mesmos tópicos ROS2
source /opt/ros/humble/setup.bash   # ou jazzy se instalado
uv run deploy --task t1_walk --mujoco
```

---

## Arquitetura

```
Host Ubuntu 24.04
├── Simulador MuJoCo  ──┐
├── RViz2               │  tópicos ROS2
└── ros2 topic list  ◄──┤  (network_mode: host)
                        │
Docker Ubuntu 22.04 ────┘
├── brain (hsl-player C++)
└── deploy (t1_walk.pt Python)
```

---

## Sobre o `booster_internal`

O `robot_client.h` usa headers de `booster_internal` (API interna da Booster, proprietária). Esses headers **não estão no repositório** e só existem no robô real.

- **Sem `booster_internal`:** o brain não compila. Os outros pacotes ROS2 (msgs, interfaces) compilam normalmente.
- **Com `booster_internal`:** monte o diretório no `docker-compose.yml`:
  ```yaml
  volumes:
    - /opt/booster_internal:/usr/local/include/booster_internal:ro
  ```

---

## ROS_DOMAIN_ID

Todos os nós (host, container, robô) devem usar o **mesmo** `ROS_DOMAIN_ID` para se enxergarem. O padrão aqui é `0`. Mude em `docker-compose.yml` e no host se necessário:

```bash
export ROS_DOMAIN_ID=0
```
