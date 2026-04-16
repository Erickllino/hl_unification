# Tópicos ROS2 do Projeto

Referência de todos os tópicos ROS2 usados no projeto, com tipo de mensagem e formato dos campos.

obs: /rl_move será o topico usado para fazer movimentação

---

## Visão Geral

```
╔══════════════════════════════════════════════════════════════════════╗
║                          BRAIN (C++)                                 ║
║                         hsl-player                                   ║
╠═══════════════════════════╦══════════════════════════════════════════╣
║  PUBLICA                  ║  ASSINA                                  ║
║  LocoApiTopicReq          ║  /booster_vision/detection               ║
║  /play_sound              ║  /booster_vision/line_segments           ║
║  /speak                   ║  /low_state                              ║
║  /rl_move  (NOVO)         ║  /odometer_state                         ║
║                           ║  /head_pose                              ║
║                           ║  fall_down_recovery_state                ║
║                           ║  /remote_controller_state                ║
║                           ║  /robocup/game_controller                ║
╚═══════════════════════════╩══════════════════════════════════════════╝

╔══════════════════════════════════════════════════════════════════════╗
║                      BOOSTER DEPLOY (Python)                         ║
║                        booster_deploy                                ║
╠═══════════════════════════╦══════════════════════════════════════════╣
║  PUBLICA                  ║  ASSINA                                  ║
║  joint_ctrl               ║  /low_state                              ║
║                           ║  /rl_move  (NOVO)                        ║
╚═══════════════════════════╩══════════════════════════════════════════╝
```

---

## Tópicos Publicados pelo Brain

---

### `LocoApiTopicReq`
**Tipo:** `booster_msgs/msg/RpcReqMsg`  
**Publicado por:** Brain  
**Consumido por:** SDK do robô (loco service interno)

Barramento geral de comandos do robô. Todos os comandos chegam neste tópico diferenciados pelo campo `api_id`.

```
RpcReqMsg {
  string uuid     # ID único da requisição (gerado automaticamente)
  string header   # JSON: { "api_id": <int> }
  string body     # JSON: parâmetros do comando (varia por api_id)
}
```

**Comandos disponíveis (api_id):**

| api_id | Nome | Body JSON |
|--------|------|-----------|
| 2000 | `kChangeMode` | `{ "mode": <int> }` — ver modos abaixo |
| 2001 | `kMove` | `{ "vx": float, "vy": float, "vyaw": float }` |
| 2004 | `kRotateHead` | `{ "pitch": float, "yaw": float }` |
| 2005 | `kWaveHand` | `{ "hand_index": int, "hand_action": int }` |
| 2007 | `kLieDown` | `{}` |
| 2008 | `kGetUp` | `{}` |
| 2024 | `kShoot` | `{}` |
| 2025 | `kGetUpWithMode` | `{ "mode": int }` |
| 2035 | `kEnterWBCGait` | `{}` |
| 2036 | `kExitWBCGait` | `{}` |
| 2038 | `kVisualKick` | `{ "start": bool }` |
| ~3xxx | `LocoInternalApiId` | kick RL, squat, robocup walk, etc. (interno) |

**Modos do robô (`kChangeMode`):**

| Valor | Modo |
|-------|------|
| kWalking | Gait embutida (kMove ativo) |
| kCustom | Controle direto dos joints via `joint_ctrl` |
| kDamping | Amortecimento — robô relaxa |

---

### `/rl_move` *(NOVO — integração t1_walk)*
**Tipo:** `geometry_msgs/msg/Twist`  
**Publicado por:** Brain  
**Consumido por:** booster_deploy (alimenta `t1_walk.pt`)

Tópico separado do `LocoApiTopicReq`. Criado para substituir o `kMove` pelo modelo de locomoção RL.

```
Twist {
  Vector3 linear
    float64 x    # velocidade para frente (m/s)
    float64 y    # velocidade lateral (m/s)
    float64 z    # não usado (0.0)
  Vector3 angular
    float64 x    # não usado (0.0)
    float64 y    # não usado (0.0)
    float64 z    # velocidade de rotação yaw (rad/s)
}
```

---

### `/play_sound`
**Tipo:** `std_msgs/msg/String`  
**Publicado por:** Brain  
**Consumido por:** nó de som

```
String {
  string data   # nome/caminho do arquivo de som
}
```

---

### `/speak`
**Tipo:** `std_msgs/msg/String`  
**Publicado por:** Brain  
**Consumido por:** nó de TTS

```
String {
  string data   # texto a ser falado
}
```

---

## Tópicos Publicados pelo Booster Deploy

---

### `joint_ctrl`
**Tipo:** `booster_interface/msg/LowCmd`  
**Publicado por:** booster_deploy  
**Consumido por:** driver dos motores

Comandos diretos de posição/torque para cada joint. Ativo apenas quando o robô está em modo `kCustom`.

```
LowCmd {
  int8 cmd_type           # 0 = CMD_TYPE_PARALLEL, 1 = CMD_TYPE_SERIAL
  MotorCmd[] motor_cmd    # um por joint (23 joints no T1)
}

MotorCmd {
  int8  mode    # modo do motor
  float32 q     # posição alvo (rad)
  float32 dq    # velocidade alvo (rad/s)
  float32 tau   # torque feedforward (Nm)
  float32 kp    # ganho proporcional (stiffness)
  float32 kd    # ganho derivativo (damping)
  float32 weight
}
```

---

## Tópicos de Sensores (publicados pelo robô/hardware)

---

### `/low_state`
**Tipo:** `booster_interface/msg/LowState`  
**Publicado por:** driver do robô  
**Consumido por:** Brain, booster_deploy

Estado completo dos joints e IMU. Principal fonte de feedback do robô.

```
LowState {
  ImuState imu_state
  MotorState[] motor_state_parallel
  MotorState[] motor_state_serial    # usado pelo deploy (23 joints T1)
}

ImuState {
  float32[3] rpy    # roll, pitch, yaw (rad)
  float32[3] gyro   # velocidade angular x, y, z (rad/s)
  float32[3] acc    # aceleração linear x, y, z (m/s²)
}

MotorState {
  int8    mode        # modo atual do motor
  float32 q           # posição atual (rad)
  float32 dq          # velocidade atual (rad/s)
  float32 ddq         # aceleração atual (rad/s²)
  float32 tau_est     # torque estimado (Nm)
  int8    temperature # temperatura (°C), range: -100 a 150
  uint32  lost        # contador de pacotes perdidos
  uint32[2] reserve   # [0] = flags de erro, [1] = freq de comunicação (Hz)
}
```

---

### `/odometer_state`
**Tipo:** `booster_interface/msg/Odometer`  
**Publicado por:** driver do robô  
**Consumido por:** Brain

Posição estimada do robô por odometria.

```
Odometer {
  float32 x      # posição x (m)
  float32 y      # posição y (m)
  float32 theta  # orientação yaw (rad)
}
```

---

### `/head_pose`
**Tipo:** `geometry_msgs/msg/Pose`  
**Publicado por:** driver do robô  
**Consumido por:** Brain

Pose atual da cabeça.

```
Pose {
  Point position
    float64 x, y, z
  Quaternion orientation
    float64 x, y, z, w
}
```

---

### `fall_down_recovery_state`
**Tipo:** `booster_interface/msg/RawBytesMsg`  
**Publicado por:** sistema de recuperação do robô  
**Consumido por:** Brain

Estado de queda/recuperação.

```
RawBytesMsg {
  char[] msg   # bytes serializados internamente
}

# Valores interpretados:
# IS_READY     = 0  (em pé, normal)
# IS_FALLING   = 1  (caindo)
# HAS_FALLEN   = 2  (caído no chão)
# IS_GETTING_UP = 3 (levantando)
```

---

### `/remote_controller_state`
**Tipo:** `booster_interface/msg/RemoteControllerState`  
**Publicado por:** driver do joystick  
**Consumido por:** Brain

Estado do controle remoto (joystick).

```
RemoteControllerState {
  uint32 event  # tipo de evento SDL2
  float32 lx    # eixo horizontal esq (-1 = esquerda, +1 = direita)
  float32 ly    # eixo vertical esq   (-1 = frente,   +1 = trás)
  float32 rx    # eixo horizontal dir
  float32 ry    # eixo vertical dir
  bool a, b, x, y           # botões ABXY
  bool lb, rb               # bumpers
  bool lt, rt               # triggers
  bool ls, rs               # clicks dos analógicos
  bool back, start          # botões back/start
  bool hat_c, hat_u, hat_d, hat_l, hat_r   # direcional (D-pad)
  bool hat_lu, hat_ld, hat_ru, hat_rd
  uint8 hat_pos
}
```

---

## Tópicos de Visão

---

### `/booster_vision/detection`
**Tipo:** `vision_interface/msg/Detections`  
**Publicado por:** nó de visão  
**Consumido por:** Brain

Objetos detectados pela câmera (bola, robôs, gol, etc.).

```
Detections {
  std_msgs/Header header
  DetectedObject[] detected_objects
  float32[] radar_x      # posições x de obstáculos (radar)
  float32[] radar_y      # posições y de obstáculos (radar)
  float32[] corner_pos   # cantos do campo detectados
}

DetectedObject {
  string  label               # ex: "ball", "robot", "goalpost"
  string  color               # cor do objeto
  float64 confidence          # confiança da detecção (0.0 a 1.0)
  int64   xmin, ymin          # bounding box na imagem (pixels)
  int64   xmax, ymax
  float32[] target_uv         # coordenadas UV no plano da imagem
  float32[] received_pos      # posição recebida raw
  float32[] position          # posição 3D no frame do robô (m)
  float32[] position_projection # projeção no plano do chão
  float32[] position_cam      # posição no frame da câmera
  int32 position_confidence   # confiança da posição estimada
}
```

---

### `/booster_vision/line_segments`
**Tipo:** `vision_interface/msg/LineSegments`  
**Publicado por:** nó de visão  
**Consumido por:** Brain

Segmentos de linha do campo detectados (usados para localização).

```
LineSegments {
  std_msgs/Header header
  float32[] coordinates      # pontos 3D das linhas no chão (m)
  float32[] coordinates_uv   # coordenadas UV na imagem
}
```

---

## Tópico do Game Controller (árbitro)

---

### `/robocup/game_controller`
**Tipo:** `game_controller_interface/msg/GameControlData`  
**Publicado por:** nó que recebe pacotes UDP do GameController  
**Consumido por:** Brain

Estado da partida enviado pelo árbitro eletrônico (RoboCup GameController).

```
GameControlData {
  char[4]  header
  uint16   version
  uint8    packet_number
  uint8    players_per_team
  uint8    game_type           # ROUNDROBIN, PLAYOFF, DROPIN
  uint8    state               # INITIAL, READY, SET, PLAYING, FINISHED
  uint8    first_half          # 1 = primeiro tempo
  uint8    kick_off_team
  uint8    secondary_state     # NORMAL, PENALTYSHOOT, OVERTIME, etc.
  char[4]  secondary_state_info
  uint8    drop_in_team
  uint16   drop_in_time
  uint16   secs_remaining      # segundos restantes no tempo
  uint16   secondary_time
  TeamInfo[2] teams            # informações dos dois times
}

TeamInfo {
  uint8    team_number
  uint8    field_player_colour
  uint8    score
  uint8    penalty_shot
  uint16   single_shots
  uint8    coach_sequence
  uint8[]  coach_message
  RobotInfo coach
  RobotInfo[] players          # até 11 jogadores
}

RobotInfo {
  uint8 penalty                # estado de penalidade do jogador
  uint8 secs_till_unpenalised
  uint8 number_of_warnings
  uint8 yellow_card_count
  uint8 red_card_count
  bool  goal_keeper
}
```

---

## Resumo para o Simulador

Para simular o ambiente completo, o simulador precisa:

**Subscrever (receber comandos):**

| Tópico | O que fazer com ele |
|--------|---------------------|
| `LocoApiTopicReq` | Parsear `api_id`: 2001=mover, 2008=levantar, 2000=trocar modo, 2024=chutar, etc. |
| `joint_ctrl` | Aplicar comandos de posição/torque diretos nos joints (modo `kCustom`) |
| `/rl_move` | Velocidade para alimentar o t1_walk.pt (quando integração RL estiver ativa) |

**Publicar (simular sensores):**

| Tópico | O que simular |
|--------|---------------|
| `/low_state` | IMU (rpy, gyro) + estado de cada joint (q, dq, tau_est) |
| `/odometer_state` | Posição x, y, theta estimada |
| `/booster_vision/detection` | Objetos detectados (bola, robôs, gol) com posição 3D |
| `/booster_vision/line_segments` | Linhas do campo para localização |
| `/head_pose` | Pose atual da cabeça |
| `fall_down_recovery_state` | Estado de queda (IS_READY normalmente) |
| `/robocup/game_controller` | Estado da partida (opcional, para testes de estratégia) |
