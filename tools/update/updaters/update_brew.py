import subprocess
import shutil
from ..utils import print_command


def update_brew():
    errors = []
    if not shutil.which("brew"):
        return errors

    cmd1 = "brew update -v"
    print_command(cmd1)
    try:
        subprocess.run(["brew", "update", "-v"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd1)
    print()
    print()

    cmd2 = "brew upgrade -v"
    print_command(cmd2)
    try:
        subprocess.run(["brew", "upgrade", "-v"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd2)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_brew()
