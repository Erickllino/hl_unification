#!/bin/bash
# entrypoint.sh — Configura o ambiente ROS2 e executa o comando

# ROS2 Humble
source /opt/ros/humble/setup.bash

# hsl-player workspace (se compilado)
if [ -f /workspace/hsl-player/install/setup.bash ]; then
    source /workspace/hsl-player/install/setup.bash
fi

# booster_robotics_sdk_python no PYTHONPATH
export PYTHONPATH="/usr/local/lib/python3.10/dist-packages:${PYTHONPATH:-}"

# uv venv
if [ -f /workspace/.venv/bin/activate ]; then
    source /workspace/.venv/bin/activate
fi

exec "$@"
