import subprocess
import shutil
from ..utils import print_command


def update_brew():
    if not shutil.which("brew"):
        return

    print_command("brew update -v")
    subprocess.run(["brew", "update", "-v"], check=True)
    print()
    print()

    print_command("brew upgrade -v")
    subprocess.run(["brew", "upgrade", "-v"], check=True)
    print()
    print()


if __name__ == "__main__":
    update_brew()
