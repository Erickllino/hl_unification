# Backlog

## [BACKLOG] Integrar branch `test/mujoco_test` do hsl-player

**Repo:** https://github.com/robocin/hsl-player/tree/test/mujoco_test
**Autoras:** giovanna-bardi12, IsabelaMoura73
**Último commit relevante:** 20/03/2026 — "fix: initializes correctly in the simulator"

### O que foi adicionado nessa branch

Novos arquivos em `src/brain/src/`:
- `main_mujoco.cpp` — entry point alternativo para rodar em simulação
- `mujoco_perception.cpp` — recebe estado do rcsssmj (joints, pose, visão, game state)
- `robot_client_mujoco.cpp` — em vez de chamar o BoosterSDK (robô real), chama `MujocoPerception` in-process

Novo launch file:
- `src/brain/launch/launch_mujoco.py` — lança o brain node conectando no TCP port 60000 (rcsssmj)

Também há mudanças em `brain.cpp` que precisam ser revisadas.

### Arquitetura alvo

```
rcsssmj (servidor MuJoCo, porta 60000)
    │ TCP + S-expressions
    ├── brain_node player_id=1  ← launch_mujoco.py
    ├── brain_node player_id=2  ← launch_mujoco.py
    └── brain_node player_id=3  ← launch_mujoco.py
         │
         └── game_controller (árbitro UDP) → tópico ROS2 → brain
```

Toda a lógica de behaviour trees, visão e game_controller do hsl-player roda intacta — só a camada de atuação troca de "BoosterSDK (robô físico)" para "rcsssmj (simulação)".

### O que fazer

- [ ] Revisar mudanças em `brain.cpp` na branch
- [ ] Clonar/fazer merge da branch `test/mujoco_test` para o `hsl-player` local
- [ ] Testar build com ROS2 colcon
- [ ] Testar 3 brain nodes conectando no rcsssmj simultaneamente
- [ ] Verificar compatibilidade com o `booster_assets` e `booster_robotics_sdk` locais
