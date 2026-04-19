# Booster Assets

Contém os modelos do robô Booster (URDF/MJCF), motion data e o pacote Python auxiliar `booster_assets` para desenvolvimento local.

---

## Configurações de Robô

### Modelos K1

| Configuração | Descrição | Formatos |
|---|---|---|
| K1 (22 DoF) | 22 DoF: cabeça 2 joints, braços 4 joints ×2, pernas 6 joints ×2 | URDF (`robots/K1/K1_22dof.urdf`), XML (`robots/K1/K1_22dof.xml`) |
| K1 Locomotion | Variante de locomoção (cabeça e braços fixos) | URDF (`robots/K1/K1_locomotion.urdf`) |
| K1 com ZED (22 DoF) | Variante 22 DoF integrada com suportes de câmera ZED | URDF (`robots/K1/K1_22dof-ZED.urdf`) |

### Modelos T1

| Configuração | Descrição | Formatos |
|---|---|---|
| T1 (23 DoF) | 23 DoF: cabeça 2, braços 4×2, cintura 1, pernas 6×2 | URDF (`robots/T1/T1_23dof.urdf`), XML (`robots/T1/T1_23dof.xml`) |
| T1 Locomotion (23 DoF) | Variante de locomoção 23 DoF | URDF (`robots/T1/T1_locomotion.urdf`), XML (`robots/T1/T1_locomotion.xml`) |
| T1 com braços 7 DoF (29 DoF) | 29 DoF: braços 7×2, cabeça 2, cintura 1, pernas 6×2 | URDF (`robots/T1/T1_29dof.urdf`) |

---

## Motion Data

A pasta `motions/` contém dados de movimento retargetados para robôs Booster. Atualmente disponíveis apenas alguns movimentos de exemplo do K1.

### Formato CSV

- Cada linha representa um frame da trajetória.
- As primeiras 7 colunas são a pose da base: posição `x, y, z` + quaternion de orientação `x, y, z, w`.
- As colunas restantes são posições de joint (radianos).

### Ordem dos Joints — K1 (`booster_assets.motions.K1_JOINT_NAMES`)

```
AAHead_yaw, Head_pitch,
ALeft_Shoulder_Pitch, Left_Shoulder_Roll, Left_Elbow_Pitch, Left_Elbow_Yaw,
ARight_Shoulder_Pitch, Right_Shoulder_Roll, Right_Elbow_Pitch, Right_Elbow_Yaw,
Left_Hip_Pitch, Left_Hip_Roll, Left_Hip_Yaw, Left_Knee_Pitch, Left_Ankle_Pitch, Left_Ankle_Roll,
Right_Hip_Pitch, Right_Hip_Roll, Right_Hip_Yaw, Right_Knee_Pitch, Right_Ankle_Pitch, Right_Ankle_Roll
```

### Ordem dos Joints — T1 (`booster_assets.motions.T1_JOINT_NAMES`)

```
AAHead_yaw, Head_pitch,
Left_Shoulder_Pitch, Left_Shoulder_Roll, Left_Elbow_Pitch, Left_Elbow_Yaw,
Right_Shoulder_Pitch, Right_Shoulder_Roll, Right_Elbow_Pitch, Right_Elbow_Yaw,
Waist,
Left_Hip_Pitch, Left_Hip_Roll, Left_Hip_Yaw, Left_Knee_Pitch, Left_Ankle_Pitch, Left_Ankle_Roll,
Right_Hip_Pitch, Right_Hip_Roll, Right_Hip_Yaw, Right_Knee_Pitch, Right_Ankle_Pitch, Right_Ankle_Roll
```

### Movimentos Disponíveis

| Arquivo | FPS | Descrição |
|---------|-----|-----------|
| `k1_fight_001_30fps.csv` | 30 | Sequência de luta |
| `k1_mj2_seg1_50fps.csv` | 50 | Segmento de dança MJ |

---

## Instalação do Pacote Python

```bash
python3 -m pip install -e .
```

Permite usar `booster_assets.motions.K1_JOINT_NAMES`, `T1_JOINT_NAMES` e utilitários de carregamento de motion data no código Python.
