import subprocess
import shutil
from ..utils import print_command


def update_python():
    if shutil.which("pyenv"):
        try:
            subprocess.run(
                ["pyenv", "update", "--help"], check=True, capture_output=True
            )
            print_command("pyenv update")
            subprocess.run(["pyenv", "update"], check=True)
            print()
            print()
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    if shutil.which("pip3"):
        print_command("pip3 install -U pip")
        subprocess.run(["pip3", "install", "-U", "pip"], check=True)
        print()
        print()


if __name__ == "__main__":
    update_python()
