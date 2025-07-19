import shutil
from ..utils import run_command


def update_apt():
    if not shutil.which("apt-get"):
        return

    run_command(["sudo", "apt", "update"], "apt: update package lists")
    run_command(["sudo", "apt", "-y", "upgrade"], "apt: upgrade packages")


if __name__ == "__main__":
    update_apt()
