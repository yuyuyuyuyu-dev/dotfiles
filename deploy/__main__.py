import argparse

from deploy.handlers.deploy_to_android import deploy_to_android
from deploy.handlers.deploy_to_linux import deploy_to_linux
from deploy.handlers.deploy_to_mac import deploy_to_mac


def main():
    parser = argparse.ArgumentParser(
        description="Deploy dotfiles to a specified platform."
    )
    subparsers = parser.add_subparsers(
        dest="platform", required=True, help="Platform to deploy to"
    )

    parser_android = subparsers.add_parser("android", help="Deploy to Android")
    parser_android.set_defaults(func=deploy_to_android)

    parser_linux = subparsers.add_parser("linux", help="Deploy to Linux")
    parser_linux.set_defaults(func=deploy_to_linux)

    parser_mac = subparsers.add_parser("mac", help="Deploy to Mac")
    parser_mac.set_defaults(func=deploy_to_mac)

    args = parser.parse_args()
    args.func()


if __name__ == "__main__":
    main()
