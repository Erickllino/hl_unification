# Booster Deploy — Referência do Framework

O `booster_deploy` é um framework leve para executar políticas de controle em robôs Booster (sim2real), MuJoCo (sim2sim) e Webots (sim2sim interno). O sistema adota abstrações modulares baseadas no IsaacLab, permitindo execução unificada de políticas em plataformas simuladas e reais.

---

## Pré-requisitos

| Ambiente | Observação |
|----------|------------|
| Booster Firmware >= v1.4 | Obrigatório para deploy no robô real |
| Python 3.10+ | Já instalado no robô |
| ROS 2 Humble | Necessário para os tópicos `/low_state` e `/joint_ctrl`. Já instalado no robô. |
| MuJoCo / Webots | Opcional — instale se for usar os respectivos simuladores |

---

## Executando Deployments

### Adicionar e listar tasks

1. Crie uma subpasta em `tasks/` para a sua task.
2. Implemente uma classe `Policy`/`PolicyCfg` e forneça um `ControllerCfg` referenciando a policy.
3. Coloque os checkpoints da policy em `models/` e referencie o caminho no config.
4. Registre o `ControllerCfg` no registry de tasks (veja as tasks existentes para o padrão de registro).
5. Verifique as tasks disponíveis:

```bash
python3 scripts/deploy.py --list
```

### Sim2Sim (MuJoCo)

- Clone o `booster_assets` (modelos do robô) e instale o helper Python:

```bash
git clone https://github.com/BoosterRobotics/booster_assets
pip install -e booster_assets
```

- Instale as dependências Python no PC:

```bash
pip install -r requirements.txt
```

- Inicie a task no MuJoCo:

```bash
python scripts/deploy.py --task <NOME_DA_TASK> --mujoco
```

### Sim2Real (Robô real)

> **IMPORTANTE:** Certifique-se de instalar o [Booster Firmware](https://booster.feishu.cn/wiki/E3q5wF5SnitXZgkY18Uc8odBnXb) >= v1.4 no robô antes de continuar.
>
> No T1 Standard Edition, instale na **motion board** (não na perception board).

1. Após testar com Sim2Sim, copie o projeto para o robô.

2. Instale o Booster Robotics SDK no robô:

```bash
git clone https://github.com/BoosterRobotics/booster_robotics_sdk
cd booster_robotics_sdk
mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j$(nproc)
sudo make install
```

3. Instale as dependências Python no robô:

```bash
pip install -r requirements.txt
```

4. Inicie o ambiente ROS 2:

```bash
source /opt/booster/BoosterRos2Interface/install/setup.bash
```

5. Execute a task no robô:

```bash
python3 scripts/deploy.py --task <NOME_DA_TASK>
```

---

## Estrutura do Repositório

```
booster_deploy/
├─ booster_deploy/           # Controllers, policies, utilitários
├─ scripts/                  # Entry-points (deploy.py)
├─ tasks/                    # Registry e configs de tasks
├─ requirements.txt          # Dependências Python
└─ fastdds_profile.xml       # Configuração padrão FastDDS para ROS 2
```

**Módulos principais:**

- `booster_deploy/`: Módulo central com abstração unificada para simuladores e robô físico. Gerencia comunicação via ROS 2 (subscreve `/low_state`, publica `/joint_ctrl`).

- `booster_deploy/robots/`: Configurações de robô. Cada arquivo define um `RobotCfg` com:
  - nomes de joints e bodies
  - posições padrão de joint
  - stiffness (`joint_stiffness`) e damping (`joint_damping`) padrão
  - limites de esforço
  - `mjcf_path` para carregamento do modelo MuJoCo
  - `prepare_state` (pose de preparo e ganhos ao entrar no modo custom)

- `tasks/`: Definições de tasks. Cada módulo contém:
  - Classe `Policy`/`PolicyCfg` com a lógica de inferência
  - Classe `ControllerCfg` com a configuração da task
  - Registro da task com uma instância de `ControllerCfg`

```
tasks/minha_task/
├─ __init__.py        # registra a task via utils.register.register_task
├─ task.py            # implementação de Policy e ControllerCfg
├─ models/            # checkpoints da policy (opcional)
└─ motions/           # primitivas de movimento (opcional)
```

---

> Para o guia prático de desenvolvimento do `t1_walk.pt` (setup local, testes de limite, integração RoboCup), veja [dev_guide.md](dev_guide.md).
