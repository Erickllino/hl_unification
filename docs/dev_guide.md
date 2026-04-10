# Booster Deploy — Guia de Desenvolvimento (T1 Walk)

Guia prático para rodar e desenvolver o `booster_deploy` com o modelo `t1_walk.pt`,
cobrindo setup local com `uv`, simulação MuJoCo e deploy no robô real.

---

## Requisitos

| Ferramenta | Versão | Onde usar |
|---|---|---|
| Python | 3.10+ (robô) / 3.13 (local) | ambos |
| [uv](https://docs.astral.sh/uv/) | qualquer | local |
| ROS 2 Humble | — | robô real |
| MuJoCo | >= 3.x | simulação local |
| Booster Firmware | >= v1.4 | robô real |

---

## Setup local (simulação MuJoCo)

### 1. Instalar uv

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2. Clonar e entrar no projeto

```bash
git clone <este-repo>
cd booster_deploy
```

### 3. Criar ambiente virtual e instalar dependências

```bash
uv venv --python 3.13
source .venv/bin/activate
uv pip install torch mujoco scipy evdev
uv pip install -e .
```

> O `-e .` instala o pacote `booster_deploy` e `tasks` em modo editável,
> necessário para que os imports funcionem corretamente.

### 4. Clonar o booster_assets (modelos do robô)

```bash
git clone https://github.com/BoosterRobotics/booster_assets booster_assets
uv pip install -e booster_assets
```

### 5. Rodar a simulação

```bash
python scripts/deploy.py --task t1_walk --mujoco
```

O MuJoCo abrirá com o robô T1. Controles pelo teclado:
- `w/s` → vx (frente/trás)
- `a/d` → vy (esquerda/direita)
- `q/e` → vyaw (rotação)
- `Espaço` → parar

---

## Testar limites com incremento automático

O script `inject_values.py` inicia a simulação e incrementa automaticamente
um dos 3 comandos a cada 10s, útil para mapear os limites do modelo.

```bash
python inject_values.py
```

Escolha qual variável (`v1=vx`, `v2=vy`, `v3=vyaw`) e a direção (`+/-`).

Limites conhecidos do `t1_walk.pt`:

| Comando | Mínimo útil | Máximo seguro |
|---|---|---|
| vx | 0.4 m/s | ~2.0 m/s |
| vy | 0.4 m/s | ~1.0 m/s |
| vyaw | 0.25 rad/s | ~2.0 rad/s |

- `vx > 1.35` → robô começa a derivar para esquerda
- `vy > 1.0` → instável
- `vyaw > 2.0` → instável

---

## Deploy no robô real (T1)

### 1. Atualizar firmware

Instalar [Booster Firmware](https://booster.feishu.cn/wiki/E3q5wF5SnitXZgkY18Uc8odBnXb) >= v1.4
na **motion board** do T1 (não na perception board).

### 2. Buildar o SDK Python no robô

```bash
git clone https://github.com/BoosterRobotics/booster_robotics_sdk
cd booster_robotics_sdk
mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j$(nproc)
sudo make install
```

### 3. Copiar o projeto para o robô

```bash
scp -r booster_deploy/ user@<IP_ROBO>:~/
```

### 4. Instalar dependências no robô

O robô já tem Python 3.10 e ROS 2 Humble. Instalar com pip:

```bash
pip install torch scipy evdev
pip install -e .
pip install -e booster_assets
```

### 5. Iniciar o ambiente ROS 2

```bash
source /opt/booster/BoosterRos2Interface/install/setup.bash
```

### 6. Rodar o deploy

```bash
python3 scripts/deploy.py --task t1_walk
```

Sequência de inicialização:
1. Pressionar `x` (ou botão X do joystick) → entra em `kCustom` mode
2. Pressionar `r` (ou botão A do joystick) → inicia o `t1_walk.pt`
3. Robô passa a responder aos comandos de velocidade

---

## Adicionar uma nova task

```
tasks/minha_task/
├─ __init__.py      # registra a task: register_task("nome", MinhaCfg())
├─ task.py          # Policy + ControllerCfg
└─ models/
   └─ modelo.pt
```

Verificar se a task foi registrada:

```bash
python scripts/deploy.py --list
```

---

## Estrutura do repositório

```
booster_deploy/
├─ booster_deploy/
│  ├─ controllers/
│  │  ├─ booster_robot_controller.py   # deploy no robô real (kCustom mode)
│  │  ├─ mujoco_controller.py          # simulação MuJoCo
│  │  └─ base_controller.py            # loop de controle comum
│  ├─ robots/
│  │  └─ booster.py                    # configuração do T1 (juntas, limites)
│  └─ utils/
│     └─ remote_control_service.py     # joystick / teclado / ROS2 /cmd_vel
├─ tasks/
│  └─ locomotion/
│     ├─ locomotion.py                 # policy do t1_walk (observação + inferência)
│     └─ models/t1_walk.pt             # checkpoint TorchScript
├─ scripts/
│  └─ deploy.py                        # entrypoint principal
├─ inject_values.py                    # teste de limites por incremento
├─ pyproject.toml
├─ requirements.txt
├─ integracao_robocup_t1walk.md        # guia de integração com robocup_demo
└─ limites_t1_walk.md                  # tabela de limites do t1_walk.pt
```

---

## Integração com robocup_demo

Para usar o `t1_walk.pt` como locomotion do `robocup_demo` (RoboCup),
consulte [integracao_robocup_t1walk.md](integracao_robocup_t1walk.md).

Resumo: o `robocup_demo` publica `vx/vy/vyaw` via `/booster/cmd_vel` (ROS2),
e o `booster_deploy` substitui o joystick por esse tópico.
