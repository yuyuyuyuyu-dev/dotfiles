import shutil
from ..utils import run_command


def update_npm():
    if not shutil.which("npm"):
        return

    run_command(["npm", "outdated", "-g", "--depth=0"], "npm: check outdated")
    run_command(["npm", "update", "-g"], "npm: update global packages")


if __name__ == "__main__":
    update_npm()
