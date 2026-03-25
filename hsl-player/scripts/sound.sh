cd `dirname $0`
cd ..

export PYTHONWARNINGS="ignore::DeprecationWarning,ignore::UserWarning,ignore::FutureWarning"
source install/setup.bash

colcon build  --symlink-install  --parallel-workers $(nproc) "$@"


(
  sleep 2
  ros2 topic pub -1 /play_sound std_msgs/String "data: 'rickroll'"

) &


espeak "Lets get this party started" >/dev/null 2>&1 || echo "Lets get this party started"
ros2 run sound_play sound_play_node

  
