import subprocess
import shutil
from ..utils import print_command


def update_volta():
    errors = []
    if not shutil.which("volta"):
        return errors

    cmd1 = "curl https://get.volta.sh | bash"
    print_command(cmd1)
    try:
        subprocess.run(cmd1, shell=True, check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd1)
    print()
    print()

    cmd2 = "volta install node"
    print_command(cmd2)
    try:
        subprocess.run(["volta", "install", "node"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd2)
    print()
    print()

    cmd3 = "volta install npm"
    print_command(cmd3)
    try:
        subprocess.run(["volta", "install", "npm"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd3)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_volta()
