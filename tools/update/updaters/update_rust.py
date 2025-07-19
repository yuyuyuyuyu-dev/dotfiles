import subprocess
import shutil
from ..utils import run_command


def update_rust():
    if shutil.which("rustup"):
        run_command(["rustup", "update"], "rustup: update")

    if shutil.which("cargo"):
        try:
            subprocess.run(
                ["cargo", "install-update", "--version"],
                check=True,
                capture_output=True,
            )
            run_command(["cargo", "install-update", "-a"], "cargo: update packages")
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass


if __name__ == "__main__":
    update_rust()
