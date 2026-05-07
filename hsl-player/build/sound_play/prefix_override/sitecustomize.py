import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/booster/Workspace/hl_unification/hsl-player/install/sound_play'
