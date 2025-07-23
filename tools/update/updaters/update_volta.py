import subprocess
import shutil
from ..utils import print_command


def update_volta():
    if not shutil.which("volta"):
        return

    print_command("curl https://get.volta.sh | bash")
    subprocess.run("curl https://get.volta.sh | bash", shell=True, check=True)
    print()
    print()

    print_command("volta install node")
    subprocess.run(["volta", "install", "node"], check=True)
    print()
    print()

    print_command("volta install npm")
    subprocess.run(["volta", "install", "npm"], check=True)
    print()
    print()


if __name__ == "__main__":
    update_volta()
