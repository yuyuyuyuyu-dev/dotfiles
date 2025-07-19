import subprocess
import shutil
from ..utils import run_command


def update_python():
    if shutil.which("pyenv"):
        try:
            subprocess.run(
                ["pyenv", "update", "--help"], check=True, capture_output=True
            )
            run_command(["pyenv", "update"], "pyenv: update")
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    if shutil.which("pip3"):
        run_command(["pip3", "install", "-U", "pip"], "pip: update pip")


if __name__ == "__main__":
    update_python()
