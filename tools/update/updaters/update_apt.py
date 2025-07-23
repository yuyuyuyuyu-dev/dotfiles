import subprocess
import shutil
from ..utils import print_command


def update_apt():
    if not shutil.which("apt-get"):
        return

    print_command("sudo apt update")
    subprocess.run(["sudo", "apt", "update"], check=True)
    print()
    print()

    print_command("sudo apt -y upgrade")
    subprocess.run(["sudo", "apt", "-y", "upgrade"], check=True)
    print()
    print()


if __name__ == "__main__":
    update_apt()
