import subprocess
import shutil
from ..utils import print_command


def update_apt():
    errors = []
    if not shutil.which("apt-get"):
        return errors

    cmd1 = "sudo apt update"
    print_command(cmd1)
    try:
        subprocess.run(["sudo", "apt", "update"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd1)
    print()
    print()

    cmd2 = "sudo apt -y upgrade"
    print_command(cmd2)
    try:
        subprocess.run(["sudo", "apt", "-y", "upgrade"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd2)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_apt()
