import subprocess
import shutil
from ..utils import print_command


def update_npm():
    if not shutil.which("npm"):
        return

    print_command("npm outdated -g --depth=0")
    subprocess.run(["npm", "outdated", "-g", "--depth=0"])
    print()
    print()

    print_command("npm update -g")
    subprocess.run(["npm", "update", "-g"], check=True)
    print()
    print()


if __name__ == "__main__":
    update_npm()
