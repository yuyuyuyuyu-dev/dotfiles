import shutil
from ..utils import run_command


def update_volta():
    if not shutil.which("volta"):
        return

    run_command(
        "curl https://get.volta.sh | bash",
        "volta: self-update",
        shell=True,
    )
    run_command(["volta", "install", "node"], "volta: install node")
    run_command(["volta", "install", "npm"], "volta: install npm")


if __name__ == "__main__":
    update_volta()
