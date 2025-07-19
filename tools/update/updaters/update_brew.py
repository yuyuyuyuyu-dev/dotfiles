import shutil
from ..utils import run_command


def update_brew():
    if not shutil.which("brew"):
        return

    run_command(["brew", "update", "-v"], "brew: update formula")
    run_command(["brew", "upgrade", "-v"], "brew: upgrade packages")


if __name__ == "__main__":
    update_brew()
