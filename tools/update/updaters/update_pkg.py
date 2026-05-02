import subprocess
import shutil
from ..utils import print_command


def update_pkg():
    errors = []
    if not shutil.which("pkg"):
        return errors

    cmd1 = "pkg upgrade -y"
    print_command(cmd1)
    try:
        subprocess.run(["pkg", "upgrade", "-y"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd1)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_pkg()
