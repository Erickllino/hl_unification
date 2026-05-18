# RL Deploy — Diagnóstico e Troubleshooting

Histórico completo dos problemas encontrados ao integrar o `booster_deploy` com o `hsl-player` no robô T1 real, com soluções aplicadas e problemas que podem ainda estar presentes.

---

## Arquitetura do pipeline

```
brain_node
    │ publica /rl_move (Twist) a 10 Hz
    ▼
rl_game_controller.py  (hsl-player --game)
    │ _rl_move_handler → synced_command (shared memory)
    │
    ├── [main process]  _low_state_handler: lê /low_state → synced_state
    │
    └── [inference process]  (mp.Process fork)
            │ lê synced_state + synced_command
            │ roda LocomotionPolicy.inference()
            ▼
        ctrl_step() → publica /joint_ctrl (LowCmd) via ROS2
            │
            ▼
        _CREATED_BY_BARE_DDS_APP_ (Booster SDK)
            │ aplica comandos nas juntas físicas
            ▼
        Hardware T1
```

---

## Problemas encontrados e soluções aplicadas

### 1. Deploy não iniciava — esperando subscriber em `/joint_ctrl`

**Sintoma:** O log ficava preso em `"Waiting for '/joint_ctrl' subscriber"` indefinidamente.

**Causa:** O SDK do robô assina `/joint_ctrl` como bare DDS (não ROS2), identificado como `_CREATED_BY_BARE_DDS_APP_`. O deploy só avançava quando esse subscriber aparecia. Na primeira execução ele não aparecia porque o robô SDK ainda não estava pronto.

**Solução:** Reiniciar o `start.sh`. Na segunda execução o SDK já está ativo e a assinatura aparece imediatamente.

---

### 2. QoS mismatch — publisher RELIABLE vs subscriber BEST_EFFORT

**Sintoma:** `/joint_ctrl` publicava mensagens mas o SDK não recebia, ou recebia de forma intermitente.

**Causa:** O deploy publicava `/joint_ctrl` com `ReliabilityPolicy.RELIABLE`, mas o SDK bare DDS assina com `BEST_EFFORT`. QoS incompatível causa falha silenciosa na entrega.

**Solução aplicada:** Em `rl_game_controller.py` e `sim_controller.py`, o publisher `/joint_ctrl` foi alterado para `BEST_EFFORT`:

```python
publisher = self.publish_node.create_publisher(
    LowCmd,
    "joint_ctrl",
    QoSProfile(
        depth=1,
        reliability=ReliabilityPolicy.BEST_EFFORT,  # ← era RELIABLE
        history=HistoryPolicy.KEEP_LAST
    )
)
```

**Arquivo afetado:** `booster_deploy/booster_deploy/controllers/rl_game_controller.py` e `sim_controller.py`

---

### 3. `MotorCmd.weight = 0.0` — robô ignorava completamente os comandos

**Sintoma:** O pipeline completo funcionava (rl_move → inference → joint_ctrl), mas o robô fisicamente não se movia.

**Causa:** O campo `weight` em `MotorCmd` é um fator de blend entre o controle interno do SDK e o comando custom. Com `weight=0.0` (valor padrão), o SDK usa 0% do comando custom → o robô ignora tudo que publicamos, mesmo em modo `kCustom`.

**Solução aplicada:** Adicionado `weight=1.0` em todos os controllers, tanto na fase de preparação (`start_custom_mode_conditionally`) quanto no loop de inferência (`ctrl_step`):

```python
# Em start_custom_mode_conditionally (fase de transição para prepare pose)
for i in range(self.robot.num_joints):
    self.motor_cmd[i].q = init_joint_pos[i]
    self.motor_cmd[i].kp = float(prepare_state.stiffness[i])
    self.motor_cmd[i].kd = float(prepare_state.damping[i])
    self.motor_cmd[i].weight = 1.0  # ← adicionado

# Em ctrl_step (loop de inferência RL)
for i in range(self.robot.num_joints):
    self.portal.motor_cmd[i].q = float(dof_targets[i].item())
    self.portal.motor_cmd[i].kp = kp_val
    self.portal.motor_cmd[i].kd = kd_val
    self.portal.motor_cmd[i].weight = 1.0  # ← adicionado
```

**Arquivos afetados:** `rl_game_controller.py`, `sim_controller.py`, `booster_robot_controller.py`

---

### 4. `stop.sh` não matava o processo `deploy.py`

**Sintoma:** Cada reinicialização do `start.sh` acumulava um novo processo `deploy.py` sem matar o anterior. Em sessões longas chegamos a **17 processos simultâneos**, todos competindo pelo controle do hardware com configurações diferentes (alguns com `weight=0`, outros com `weight=1`).

**Causa:** O `stop.sh` original só matava os nós ROS2 (`brain_node`, `vision_node`, etc.), mas não o processo Python `deploy.py` que é lançado separadamente via `nohup`.

**Solução aplicada:** Adicionado `pkill -9 -f "deploy.py"` ao `stop.sh`:

```bash
echo ["STOP DEPLOY"]
pkill -9 -f "deploy.py"
```

O `-9` (SIGKILL) é necessário porque o processo captura SIGTERM e aguarda o subprocesso de inferência terminar, o que pode não acontecer se o subprocesso travar.

**Arquivo afetado:** `hsl-player/scripts/stop.sh`

> **Nota:** Se ainda houver processos velhos rodando, mate manualmente com:
> ```bash
> pkill -9 -f "deploy.py"
> ```

---

### 5. Modelo errado (`t1_test` vs `t1_walk`)

**Sintoma:** O robô se movia muito pouco — targets de joelho variavam de 0.177 a 0.155 rad (diferença de apenas 0.022 rad) mesmo com vx=1.0.

**Causa:** O `start.sh` usava `--task t1_test`, que carrega `models/policy.pt` via `T1WalkTest(T1TestController2)`. Esse modelo produz ações quase estáticas (~0.25 action_scale × ~1.0 action = 0.25 rad de desvio máximo do default). Na simulação com `t1_walk`, o joelho oscila entre 0.265 e 0.678 rad — ações muito maiores e dinâmicas, características de um gait real.

**Solução aplicada:** Alterado `start.sh` para usar `--task t1_walk`:

```bash
nohup python3 scripts/deploy.py --task t1_walk --auto-start --game ...
```

`t1_walk` usa `models/t1_walk.pt` com `T1WalkControllerCfg` (kp=200 para pernas, action_scale=0.075 para joelho).

**Arquivo afetado:** `hsl-player/scripts/start.sh`

---

### 6. `ChangeMode(kCustom)` chamado apenas uma vez, sem verificação

**Sintoma:** O robô não entrava em custom mode automaticamente no deploy, mas entrava quando o modo era trocado manualmente.

**Causa:** O código chamava `self.client.ChangeMode(RobotMode.kCustom)` uma única vez sem verificar se o comando foi aceito. Se o robô não estiver em estado adequado (ex: ainda inicializando), a chamada falha silenciosamente e o robô permanece em outro modo. Como o deploy não verifica isso, o loop de RL começa a publicar comandos que são completamente ignorados.

**Solução aplicada:** Substituído por um loop de retry com verificação:

```python
for i in range(20):
    self.client.ChangeMode(RobotMode.kCustom)
    time.sleep(0.5)
    mode = self.client.GetStatus().current_mode
    self.logger.info(f"ChangeMode attempt {i+1}: current_mode={mode}")
    if mode == RobotMode.kCustom:
        self.logger.info("Successfully switched to kCustom mode")
        break
else:
    self.logger.error("Failed to switch to kCustom after 20 attempts")
    return False
```

**Arquivo afetado:** `hsl-player/scripts/rl_game_controller.py`

---

### 7. Comanda `/rl_move` sem `-r 10` era ignorada

**Sintoma:** Publicar um único comando de velocidade via `ros2 topic pub /rl_move ...` (sem `-r`) não fazia o robô andar.

**Causa:** O `_low_state_handler` tem um timeout (`_rl_move_timeout = 0.5s`). Se nenhuma mensagem nova de `/rl_move` chegar dentro desse período, o handler sobrescreve o `synced_command` com zeros. Uma mensagem única publicada sem `-r` era consumida e o comando zerava antes da inferência ler.

**Solução:** Sempre usar `-r 10` para publicação contínua:

```bash
ros2 topic pub /rl_move geometry_msgs/msg/Twist \
  '{linear: {x: 0.5, y: 0.0, z: 0.0}, angular: {x: 0.0, y: 0.0, z: 0.0}}' -r 10
```

---

## Monitorar o deploy em tempo real

O log do deploy é gravado em `hsl-player/deploy.log`. Para acompanhar em tempo real:

```bash
tail -f ~/Workspace/hl_unification/hsl-player/deploy.log
```

### O que observar no log

| Linha esperada | Significado |
|---|---|
| `Subscriber found, starting control loop` | SDK do robô está online e assinou `/joint_ctrl` |
| `ChangeMode attempt N: current_mode=RobotMode.kCustom` | Modo custom ativo — comandos serão aceitos |
| `Successfully switched to kCustom mode` | Fase de preparação completada com sucesso |
| `Custom mode started, initialized with prepare pose` | Transição para pose inicial concluída |
| `Inference process started` | Processo de inferência RL iniciado |
| `[DEBUG ctrl_step] step=N left_leg_targets=[...]` | Targets sendo enviados ao hardware (a cada 100 passos) |
| `[DEBUG policy] vel_in_obs=[1.0, ...]` | Velocidade chegando na inferência corretamente |

### Sinais de problema

| Linha | Problema |
|---|---|
| `Waiting for '/joint_ctrl' subscriber` por mais de 10s | SDK do robô não está ativo — verifique o firmware |
| `ChangeMode attempt 20` sem sucesso | Robô não aceita kCustom — verifique estado do robô (deve estar em pé, estável) |
| `[DEBUG inference] vel_cmd vx=0.000` apesar de publicar | Timeout do /rl_move — use `-r 10` na publicação |
| `[DEBUG ctrl_step]` com targets estáticos e pequenos (~0.15 para joelho) | Usando `t1_test` em vez de `t1_walk` — verifique o `start.sh` |
| step counter com saltos (ex: 3800 → 82000) | Múltiplos processos deploy.py rodando — execute `pkill -9 -f "deploy.py"` |

---

## Problemas que podem ainda estar ocorrendo

### A. Sim2real gap — política não generaliza perfeitamente para o hardware real

O modelo `t1_walk.pt` foi treinado em simulação. No hardware real, os parâmetros de atuador (kp, kd, friction, delays) nunca são idênticos. O robô pode:
- Andar com gait instável ou assimétrico
- Derivar lateralmente mesmo com vy=0
- Cair em velocidades que eram estáveis na sim

**O que fazer:**
- Comece com vx baixo (0.3–0.5 m/s) e aumente gradualmente
- Mantenha alguém pronto para segurar o robô
- Consulte `docs/limites_t1_walk.md` para os limites conhecidos

### B. `ChangeMode(kCustom)` pode falhar se o robô não estiver no estado correto

O robô precisa estar em modo `kWalking` (em pé, estável) antes de aceitar `kCustom`. Se iniciado com o robô deitado, agachado ou em `kDamping`, a troca pode falhar mesmo com o retry de 20 tentativas. O log mostrará `"Failed to switch to kCustom after 20 attempts"` e o deploy será cancelado.

**O que fazer:** Garanta que o robô está em pé e estável (modo padrão de operação) antes de rodar o `start.sh`.

### C. Timeout do `/rl_move` causa oscilação de velocidade

O `_low_state_handler` sobrescreve o `synced_command` com zeros quando não chega mensagem de `/rl_move` há mais de `_rl_move_timeout` segundos. O processo de inferência lê o comando a ~50 Hz, então se o ROS2 tiver latência ou perda de pacotes, a inferência pode ver velocidade zero intercalada com a velocidade real, causando comportamento inesperado.

**O que fazer:** Verifique o tópico `/rl_move` antes de testar:
```bash
ros2 topic hz /rl_move   # deve mostrar ~10 Hz
ros2 topic echo /rl_move --once  # confirma conteúdo
```

### D. Shared memory residual entre reinicializações

O `SyncedArray` usa `multiprocessing.shared_memory` com nome baseado no PID do processo pai. Se o processo for morto com SIGKILL (`pkill -9`), os blocos de shared memory ficam no sistema (`/dev/shm/`) e não são limpos. Em situações raras, um nome pode colidir com um bloco residual e causar leitura de dados antigos.

**O que fazer:** Se o comportamento parecer errado logo após a inicialização, limpe manualmente:
```bash
ls /dev/shm/ | grep -E "action|state|command"
# Se houver arquivos órfãos:
rm /dev/shm/action_* /dev/shm/state_* /dev/shm/command_*
```

### E. Debug prints no código de produção

Foram adicionados vários `print` de debug nos arquivos de inferência para diagnosticar os problemas acima. Eles geram overhead e ruído no log em produção.

**Localização dos prints de debug:**
- `tasks/locomotion/locomotion.py`: `[DEBUG policy]` — mostra vel_in_obs e leg_actions a cada 100 passos
- `booster_deploy/controllers/rl_game_controller.py`: `[DEBUG ctrl_step]` e `[DEBUG inference]` — mostra targets e velocidade
- `booster_deploy/controllers/rl_game_controller.py`: `[DEBUG rl_move_handler]` — mostra cada mensagem recebida

**O que fazer quando estiver estável:** Remover os blocos de debug de `locomotion.py` e `rl_game_controller.py`.

---

## Referência rápida

```bash
# Iniciar tudo
cd ~/Workspace/hl_unification/hsl-player && ./scripts/start.sh

# Parar tudo (incluindo deploy)
./scripts/stop.sh

# Matar só o deploy (caso stop.sh não funcione)
pkill -9 -f "deploy.py"

# Monitorar log do deploy
tail -f ~/Workspace/hl_unification/hsl-player/deploy.log

# Verificar quantos deploy estão rodando
ps aux | grep "deploy.py" | grep -v grep

# Enviar velocidade de teste (sempre com -r 10)
ros2 topic pub /rl_move geometry_msgs/msg/Twist \
  '{linear: {x: 0.5, y: 0.0, z: 0.0}, angular: {x: 0.0, y: 0.0, z: 0.0}}' -r 10

# Verificar se /joint_ctrl está sendo publicado
ros2 topic hz /joint_ctrl

# Verificar subscribers de /joint_ctrl (deve ter _CREATED_BY_BARE_DDS_APP_)
ros2 topic info /joint_ctrl --verbose
```
