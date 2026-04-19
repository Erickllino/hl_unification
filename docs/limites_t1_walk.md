# Limites de Velocidade — t1_walk.pt

Referência dos limites de comando para o modelo `t1_walk.pt` (locomotion RL do Booster T1).

---

## Convenção de Direções

| Variável | Positivo | Negativo |
|----------|----------|----------|
| `vx` | Frente | Trás |
| `vy` | Esquerda | Direita |
| `vyaw` | Anti-horário | Horário |

---

## Limites dos Comandos

| Comando | Mín. Negativo | Mín. Positivo | Máx. Positivo | Máx. Negativo |
|---------|:-------------:|:-------------:|:-------------:|:-------------:|
| `vx` (m/s) | -1.6 | 0.4 | 2.8 | -0.5 |
| `vy` (m/s) | -1.8 | 0.4 | 1.15 | -0.45 |
| `vyaw` (rad/s) | -5.2 | 0.25 | 4.60 | -0.3 |

> Comandos abaixo do mínimo são ignorados pelo modelo (sem movimento).

---

## Faixas de Instabilidade

| Variável | Condição | Comportamento Observado |
|----------|----------|------------------------|
| `vx` | ≥ 1.35 m/s | Robô começa a derivar para esquerda |
| `vx` | ≥ 2.0 m/s | Instável |
| `vx` | ≤ -0.8 m/s | Robô começa a derivar para direita |
| `vx` | ≤ -1.5 m/s | Instável |
| `vy` | ≥ 1.0 m/s | Instável |
| `vy` | ≤ -1.0 m/s | Instável |
| `vyaw` | ≥ 2.0 rad/s | Instável |
| `vyaw` | ≤ -2.15 rad/s | Instável |

---

## Testar Limites (simulação)

O script `inject_values.py` incrementa um dos comandos automaticamente a cada 10s — útil para mapear os limites em simulação MuJoCo:

```bash
cd booster_deploy
uv run python inject_values.py
```

Escolha qual variável (`v1=vx`, `v2=vy`, `v3=vyaw`) e a direção (`+/-`) ao iniciar.
