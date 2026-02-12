import subprocess
import shutil
from ..utils import print_command


def update_npm():
    errors = []
    if not shutil.which("npm"):
        return errors

    print_command("npm outdated -g --depth=0")
    subprocess.run(["npm", "outdated", "-g", "--depth=0"])
    print()
    print()

    cmd = "npm update -g"
    print_command(cmd)
    try:
        subprocess.run(["npm", "update", "-g"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd)
    print()
    print()

    return errors


if __name__ == "__main__":
    update_npm()
