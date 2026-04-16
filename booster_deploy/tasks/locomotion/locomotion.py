from __future__ import annotations
from dataclasses import MISSING
import json
import os
from pathlib import Path
import torch

from booster_deploy.controllers.base_controller import BaseController, Policy
from booster_deploy.controllers.controller_cfg import (
    ControllerCfg, PolicyCfg, VelocityCommandCfg
)
from booster_deploy.robots.booster import K1_CFG, T1_23DOF_CFG
from booster_deploy.utils.isaaclab.configclass import configclass
from booster_deploy.utils.isaaclab import math as lab_math


class LocomotionPolicy(Policy):
    """walking policy with observation history."""

    def __init__(self, cfg: LocomotionPolicyCfg, controller: BaseController):
        super().__init__(cfg, controller)
        self.cfg = cfg
        self.robot = controller.robot

        # Load policy model
        policy_path = self.cfg.checkpoint_path
        if not os.path.isabs(policy_path):
            # Try relative to task directory
            policy_path = os.path.join(self.task_path, self.cfg.checkpoint_path)

        self._model: torch.jit.ScriptModule = torch.jit.load(
            policy_path, map_location="cpu")
        self._model.eval()

        # Observation and action parameters
        self.actor_obs_history_length = cfg.actor_obs_history_length
        if isinstance(cfg.action_scale, (list, tuple)):
            self.action_scale = torch.tensor(cfg.action_scale, dtype=torch.float32)
        else:
            self.action_scale = cfg.action_scale

        # Initialize buffers
        self.obs_history = None
        self.last_action = torch.zeros(
            len(self.cfg.policy_joint_names), dtype=torch.float32)

        self.real2sim_joint_map = torch.tensor([
            self.robot.cfg.joint_names.index(name)
            for name in self.cfg.policy_joint_names
        ], dtype=torch.long)

    def reset(self) -> None:
        """Initialize policy state."""
        pass

    def compute_observation(self) -> torch.Tensor:
        """Compute current observation following sim2sim.py pattern."""
        # Get robot state
        dof_pos = self.robot.data.joint_pos
        dof_vel = self.robot.data.joint_vel
        base_quat = self.robot.data.root_quat_w
        base_ang_vel = self.robot.data.root_ang_vel_b

        # Project gravity vector into base frame
        gravity_w = torch.tensor([0.0, 0.0, -1.0], dtype=torch.float32)
        projected_gravity = lab_math.quat_apply_inverse(base_quat, gravity_w)

        if self.cfg.enable_safety_fallback:
            # fall detection: stop if falling
            if projected_gravity[2] > -0.5:
                print("\nFalling detected, stopping policy for safety. "
                      "You can disable safety fallback by setting "
                      f"{self.cfg.__class__.__name__}.enable_safety_fallback "
                      "to False.")
                self.controller.stop()

        # Get velocity commands
        lin_vel_x = self.controller.vel_command.lin_vel_x
        lin_vel_y = self.controller.vel_command.lin_vel_y
        ang_vel_yaw = self.controller.vel_command.ang_vel_yaw

        default_joint_pos_sim = self.robot.default_joint_pos
        mapped_default_pos = default_joint_pos_sim[self.real2sim_joint_map]
        mapped_dof_pos = dof_pos[self.real2sim_joint_map]
        mapped_dof_vel = dof_vel[self.real2sim_joint_map]

        # Build observation: [
        #   ang_vel(3),
        #   projected_gravity(3),
        #   commands(3),
        #   joint_pos(num_action),
        #   joint_vel(num_action),
        #   actions(num_action)]

        obs = torch.cat([
            base_ang_vel,
            projected_gravity,
            torch.tensor(
                [lin_vel_x, lin_vel_y, ang_vel_yaw], dtype=torch.float32),
            (mapped_dof_pos - mapped_default_pos) * 1.0,
            mapped_dof_vel * self.cfg.obs_dof_vel_scale,
            self.last_action * 1.0
        ], dim=0)

        return obs

    def inference(self) -> torch.Tensor:
        """Compute action from policy."""
        # Compute current observation
        obs = self.compute_observation()

        if self.obs_history is None:
            # Warm-start: fill the entire history with the first real observation
            # instead of zeros. Avoids extreme first-step actions caused by the
            # policy receiving 9 all-zero frames + 1 real gravity frame, which
            # would produce large joint targets and destabilize the robot.
            self.obs_history = obs.clamp(-100.0, 100.0).unsqueeze(0).expand(
                self.actor_obs_history_length, -1
            ).clone()

        # Update observation history (roll and append)
        self.obs_history = self.obs_history.roll(shifts=-1, dims=0)
        self.obs_history[-1] = obs.clamp(-100.0, 100.0)

        # Get action from policy
        with torch.no_grad():
            action = self._model(self.obs_history.flatten()).squeeze(0)
            action = torch.clamp(action, -100.0, 100.0)

        # Store action for next step
        self.last_action = action.clone()

        default_joint_pos = self.robot.default_joint_pos

        dof_targets = default_joint_pos.clone()
        dof_targets.scatter_reduce_(
            0,
            self.real2sim_joint_map,
            action * self.action_scale,
            reduce='sum')
        return dof_targets


@configclass
class LocomotionPolicyCfg(PolicyCfg):
    constructor = LocomotionPolicy
    checkpoint_path: str = MISSING  # type: ignore
    actor_obs_history_length: int = 10
    action_scale: float | list[float] = 0.25
    obs_dof_vel_scale: float = 1.0
    policy_joint_names: list[str] = MISSING  # type: ignore


@configclass
class K1WalkControllerCfg(ControllerCfg):
    robot = K1_CFG.replace(  # type: ignore
        default_joint_pos=[
            0, 0,
            0.2, -1.25, 0, -0.5,
            0.2,  1.25, 0,  0.5,
            -0.15, 0, 0, 0.3, -0.15, 0.,
            -0.15, 0, 0, 0.3, -0.15, 0.
        ],
        joint_stiffness=[
            4.0, 4.0,
            20.0, 20.0, 20.0, 20.0,
            20.0, 20.0, 20.0, 20.0,
            100.0, 100.0, 100.0, 100.0, 50.0, 50.0,
            100.0, 100.0, 100.0, 100.0, 50.0, 50.0,
        ],
        joint_damping=[
            1.0, 1.0,
            2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0,
            2.0, 2.0, 2.0, 2.0, 1.0, 1.0,
            2.0, 2.0, 2.0, 2.0, 1.0, 1.0,
        ],
    )
    vel_command: VelocityCommandCfg = VelocityCommandCfg(
        vx_max=1.0,
        vy_max=1.0,
        vyaw_max=1.0,
    )
    policy: LocomotionPolicyCfg = LocomotionPolicyCfg(
        obs_dof_vel_scale=0.1,
        policy_joint_names=[
            "ALeft_Shoulder_Pitch",
            "ARight_Shoulder_Pitch",
            "Left_Hip_Pitch",
            "Right_Hip_Pitch",
            "Left_Shoulder_Roll",
            "Right_Shoulder_Roll",
            "Left_Hip_Roll",
            "Right_Hip_Roll",
            "Left_Elbow_Pitch",
            "Right_Elbow_Pitch",
            "Left_Hip_Yaw",
            "Right_Hip_Yaw",
            "Left_Elbow_Yaw",
            "Right_Elbow_Yaw",
            "Left_Knee_Pitch",
            "Right_Knee_Pitch",
            "Left_Ankle_Pitch",
            "Right_Ankle_Pitch",
            "Left_Ankle_Roll",
            "Right_Ankle_Roll",
        ],
    )

@configclass
class T1WalkControllerCfg(ControllerCfg):
    robot = T1_23DOF_CFG.replace(  # type: ignore
        default_joint_pos=[
            0, 0,
            0.2, -1.3, 0, -0.5,
            0.2,  1.3, 0,  0.5,
            0.,
            -0.2, 0, 0, 0.4, -0.2, 0.,
            -0.2, 0, 0, 0.4, -0.2, 0.
        ],
        joint_stiffness=[
            4.0, 4.0,
            50.0, 50.0, 50.0, 50.0,
            50.0, 50.0, 50.0, 50.0,
            200.,
            200.0, 200.0, 200.0, 200.0, 50.0, 50.0,
            200.0, 200.0, 200.0, 200.0, 50.0, 50.0,
        ],
        joint_damping=[
            1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            5.0,
            5.0, 5.0, 5.0, 5.0, 2.0, 2.0,
            5.0, 5.0, 5.0, 5.0, 2.0, 2.0,
        ],
    )
    vel_command: VelocityCommandCfg = VelocityCommandCfg(
        vx_max=1.0,
        vy_max=1.0,
        vyaw_max=1.0,
    )
    policy: LocomotionPolicyCfg = LocomotionPolicyCfg(
        obs_dof_vel_scale=1.0,
        # Per-joint action scales matching training: 0.25 * effort_limit_sim / stiffness
        # arms(Shoulder/Elbow): 0.25*18/50=0.09  waist: 0.25*25/200=0.03125
        # hip_pitch: 0.25*45/200=0.05625  hip_roll/yaw: 0.25*25/200=0.03125
        # knee: 0.25*60/200=0.075  ankle_pitch: 0.25*24/50=0.12  ankle_roll: 0.25*15/50=0.075
        #
        # NEW order (mjlab_playground XML order) — use for models trained with mjlab_playground.
        # To revert to old models, comment this block and uncomment the LEGACY block below.
        # LEGACY order — for models trained before mjlab_playground migration.
        # Uncomment below and comment the blocks above to test old models.
        policy_joint_names=[
            'Left_Shoulder_Pitch',
            'Right_Shoulder_Pitch',
            'Waist',
            'Left_Shoulder_Roll',
            'Right_Shoulder_Roll',
            'Left_Hip_Pitch',
            'Right_Hip_Pitch',
            'Left_Elbow_Pitch',
            'Right_Elbow_Pitch',
            'Left_Hip_Roll',
            'Right_Hip_Roll',
            'Left_Elbow_Yaw',
            'Right_Elbow_Yaw',
            'Left_Hip_Yaw',
            'Right_Hip_Yaw',
            'Left_Knee_Pitch',
            'Right_Knee_Pitch',
            'Left_Ankle_Pitch',
            'Right_Ankle_Pitch',
            'Left_Ankle_Roll',
            'Right_Ankle_Roll',
        ],
    )

@configclass
class T1TestController2(ControllerCfg):
    """T1 controller using training actuator parameters directly.

    All values derived from t1_constants.py actuator model — no guessing needed:
      kp    = reflected_inertia * NATURAL_FREQ^2          (NATURAL_FREQ = 5*2π rad/s)
      kv    = 4 * reflected_inertia * NATURAL_FREQ        (DAMPING_RATIO = 2)
      scale = 0.25 * effort_limit / kp

    Actuator → kp / kv / scale:
      NECK            (I=0.0018):   kp=1.78   kv=0.23   scale=0.985
      ARM             (I=0.02825):  kp=27.9   kv=3.55   scale=0.323
      WAIST/HIP_RY    (I=0.04781):  kp=47.2   kv=6.01   scale=0.212
      HIP_PITCH       (I=0.05239):  kp=51.7   kv=6.58   scale=0.266
      KNEE            (I=0.06360):  kp=62.8   kv=7.99   scale=0.259
      ANKLE           (I=0.03396):  kp=33.5   kv=4.27   scale=0.373
    """
    robot = T1_23DOF_CFG.replace(  # type: ignore
        # HOME_KEYFRAME from t1_constants.py
        default_joint_pos=[
            0., 0.,
            0., -1.4, 0., -0.4,
            0.,  1.4, 0.,  0.4,
            0.,
            -0.2, 0., 0., 0.4, -0.2, 0.,
            -0.2, 0., 0., 0.4, -0.2, 0.,
        ],
        # kp = reflected_inertia * (5*2π)²
        joint_stiffness=[
            1.78, 1.78,                              # Head (NECK)
            27.9, 27.9, 27.9, 27.9,                 # Left arm (ARM)
            27.9, 27.9, 27.9, 27.9,                 # Right arm (ARM)
            47.2,                                    # Waist (WAIST_HIP_RY)
            51.7, 47.2, 47.2, 62.8, 33.5, 33.5,    # Left leg: HP HR HY K AP AR
            51.7, 47.2, 47.2, 62.8, 33.5, 33.5,    # Right leg
        ],
        # kv = 4 * reflected_inertia * (5*2π)
        joint_damping=[
            0.23, 0.23,                              # Head
            3.55, 3.55, 3.55, 3.55,                 # Left arm
            3.55, 3.55, 3.55, 3.55,                 # Right arm
            6.01,                                    # Waist
            6.58, 6.01, 6.01, 7.99, 4.27, 4.27,    # Left leg
            6.58, 6.01, 6.01, 7.99, 4.27, 4.27,    # Right leg
        ],
    )
    vel_command: VelocityCommandCfg = VelocityCommandCfg(
        vx_max=1.0,
        vy_max=1.0,
        vyaw_max=1.0,
    )
    policy: LocomotionPolicyCfg = LocomotionPolicyCfg(
        obs_dof_vel_scale=1.0,
        # scale = 0.25 * effort_limit / kp  (exact training values)
        action_scale=[
            0.323,   # Left_Shoulder_Pitch  (ARM)
            0.323,   # Left_Shoulder_Roll
            0.323,   # Left_Elbow_Pitch
            0.323,   # Left_Elbow_Yaw
            0.323,   # Right_Shoulder_Pitch
            0.323,   # Right_Shoulder_Roll
            0.323,   # Right_Elbow_Pitch
            0.323,   # Right_Elbow_Yaw
            0.212,   # Waist               (WAIST_HIP_RY)
            0.266,   # Left_Hip_Pitch      (HIP_PITCH)
            0.212,   # Left_Hip_Roll       (WAIST_HIP_RY)
            0.212,   # Left_Hip_Yaw
            0.259,   # Left_Knee_Pitch     (KNEE)
            0.373,   # Left_Ankle_Pitch    (ANKLE)
            0.373,   # Left_Ankle_Roll
            0.266,   # Right_Hip_Pitch
            0.212,   # Right_Hip_Roll
            0.212,   # Right_Hip_Yaw
            0.259,   # Right_Knee_Pitch
            0.373,   # Right_Ankle_Pitch
            0.373,   # Right_Ankle_Roll
        ],
        policy_joint_names=[
            'Left_Shoulder_Pitch',
            'Left_Shoulder_Roll',
            'Left_Elbow_Pitch',
            'Left_Elbow_Yaw',
            'Right_Shoulder_Pitch',
            'Right_Shoulder_Roll',
            'Right_Elbow_Pitch',
            'Right_Elbow_Yaw',
            'Waist',
            'Left_Hip_Pitch',
            'Left_Hip_Roll',
            'Left_Hip_Yaw',
            'Left_Knee_Pitch',
            'Left_Ankle_Pitch',
            'Left_Ankle_Roll',
            'Right_Hip_Pitch',
            'Right_Hip_Roll',
            'Right_Hip_Yaw',
            'Right_Knee_Pitch',
            'Right_Ankle_Pitch',
            'Right_Ankle_Roll',
        ]
    )


@configclass
class T1TestController(ControllerCfg):
    robot = T1_23DOF_CFG.replace(  # type: ignore
        default_joint_pos=[
            0, 0,
            0.2, -1.3, 0, -0.5,
            0.2,  1.3, 0,  0.5,
            0.,
            -0.2, 0, 0, 0.4, -0.2, 0.,
            -0.2, 0, 0, 0.4, -0.2, 0.
        ],
        joint_stiffness=[
            4.0, 4.0,
            50.0, 50.0, 50.0, 50.0,
            50.0, 50.0, 50.0, 50.0,
            200.,
            200.0, 200.0, 200.0, 200.0, 50.0, 50.0,
            200.0, 200.0, 200.0, 200.0, 50.0, 50.0,
        ],
        joint_damping=[
            1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            5.0,
            5.0, 5.0, 5.0, 5.0, 2.0, 2.0,
            5.0, 5.0, 5.0, 5.0, 2.0, 2.0,
        ],
    )
    vel_command: VelocityCommandCfg = VelocityCommandCfg(
        vx_max=1.0,
        vy_max=1.0,
        vyaw_max=1.0,
    )
    policy: LocomotionPolicyCfg = LocomotionPolicyCfg(
        obs_dof_vel_scale=1.0,
        # Per-joint action scales matching training: 0.25 * effort_limit_sim / stiffness
        # arms(Shoulder/Elbow): 0.25*18/50=0.09  waist: 0.25*25/200=0.03125
        # hip_pitch: 0.25*45/200=0.05625  hip_roll/yaw: 0.25*25/200=0.03125
        # knee: 0.25*60/200=0.075  ankle_pitch: 0.25*24/50=0.12  ankle_roll: 0.25*15/50=0.075
        #
        # NEW order (mjlab_playground XML order) — use for models trained with mjlab_playground.
        # To revert to old models, comment this block and uncomment the LEGACY block below.
        action_scale=[
            0.09,     # Left_Shoulder_Pitch
            0.09,     # Left_Shoulder_Roll
            0.09,     # Left_Elbow_Pitch
            0.09,     # Left_Elbow_Yaw
            0.09,     # Right_Shoulder_Pitch
            0.09,     # Right_Shoulder_Roll
            0.09,     # Right_Elbow_Pitch
            0.09,     # Right_Elbow_Yaw
            0.03125,  # Waist
            0.05625,  # Left_Hip_Pitch
            0.03125,  # Left_Hip_Roll
            0.03125,  # Left_Hip_Yaw
            0.075,    # Left_Knee_Pitch
            0.12,     # Left_Ankle_Pitch
            0.075,    # Left_Ankle_Roll
            0.05625,  # Right_Hip_Pitch
            0.03125,  # Right_Hip_Roll
            0.03125,  # Right_Hip_Yaw
            0.075,    # Right_Knee_Pitch
            0.12,     # Right_Ankle_Pitch
            0.075,    # Right_Ankle_Roll
        ],
        # policy_joint_names=[
        #     'Left_Shoulder_Pitch',
        #     'Right_Shoulder_Pitch',
        #     'Waist',
        #     'Left_Shoulder_Roll',
        #     'Right_Shoulder_Roll',
        #     'Left_Hip_Pitch',
        #     'Right_Hip_Pitch',
        #     'Left_Elbow_Pitch',
        #     'Right_Elbow_Pitch',
        #     'Left_Hip_Roll',
        #     'Right_Hip_Roll',
        #     'Left_Elbow_Yaw',
        #     'Right_Elbow_Yaw',
        #     'Left_Hip_Yaw',
        #     'Right_Hip_Yaw',
        #     'Left_Knee_Pitch',
        #     'Right_Knee_Pitch',
        #     'Left_Ankle_Pitch',
        #     'Right_Ankle_Pitch',
        #     'Left_Ankle_Roll',
        #     'Right_Ankle_Roll',
        # ],
        policy_joint_names=[
            'Left_Shoulder_Pitch',
            'Left_Shoulder_Roll',
            'Left_Elbow_Pitch',
            'Left_Elbow_Yaw',
            'Right_Shoulder_Pitch',
            'Right_Shoulder_Roll',
            'Right_Elbow_Pitch',
            'Right_Elbow_Yaw',
            'Waist',
            'Left_Hip_Pitch',
            'Left_Hip_Roll',
            'Left_Hip_Yaw',
            'Left_Knee_Pitch',
            'Left_Ankle_Pitch',
            'Left_Ankle_Roll',
            'Right_Hip_Pitch',
            'Right_Hip_Roll',
            'Right_Hip_Yaw',
            'Right_Knee_Pitch',
            'Right_Ankle_Pitch',
            'Right_Ankle_Roll',
        ]
    )


