import sys


def main():
    import argparse
    import pkgutil

    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--task", type=str, help="Name of the task to run.")
    group.add_argument("-l", "--list", action="store_true", dest="list_tasks",
                       default=False, help="list available tasks")

    parser.add_argument("--net", type=str, default="127.0.0.1",
                        help="Network interface for SDK communication.")
    parser.add_argument("--mujoco", action="store_true", default=False,
                        help="deploy in mujoco simulation")
    parser.add_argument("--webots", action="store_true", default=False,
                        help="deploy in webots simulation")
    parser.add_argument("--device", type=str, default="cpu",
                        help="Device to run the evaluation on (e.g., 'cpu', 'cuda')")
    args = parser.parse_args()

    # auto-import all submodules under tasks so they can register themselves
    import tasks as tasks_pkg
    for mod_info in pkgutil.walk_packages(tasks_pkg.__path__, prefix="tasks."):
        try:
            __import__(mod_info.name)
        except Exception as e:
            raise e

    from booster_deploy.utils.registry import get_task, list_tasks

    if args.list_tasks:
        print("Available tasks:")
        for task_name, cfg in list_tasks().items():
            cls = type(cfg)
            full_cls = f"{cls.__module__}.{cls.__qualname__}"
            print(f"  {task_name}\t:\t{full_cls}")
        sys.exit(0)

    try:
        task_cfg = get_task(args.task)
    except KeyError:
        print(f"Unknown task '{args.task}'. Available tasks: {list(list_tasks().keys())}")
        sys.exit(1)

    task_cfg.policy.device = args.device

    if args.mujoco:
        from booster_deploy.controllers.mujoco_controller import MujocoController
        MujocoController(task_cfg).run()
    else:
        try:
            from booster_robotics_sdk_python import ChannelFactory  # type: ignore
            ChannelFactory.Instance().Init(0, args.net)
        except ImportError:
            print(
                "Error: booster_robotics_sdk_python is not installed.\n"
                "Please install it to use real robot deployment.\n"
                "For MuJoCo simulation, use --mujoco flag instead."
            )
            sys.exit(1)

        if args.webots:
            ankles = [-8, -7, -2, -1]
            for i in ankles:
                task_cfg.robot.joint_damping[i] = 0.5

        from booster_deploy.controllers.booster_robot_controller import BoosterRobotPortal
        with BoosterRobotPortal(task_cfg, use_sim_time=args.webots) as portal:
            portal.run()


if __name__ == "__main__":
    main()
