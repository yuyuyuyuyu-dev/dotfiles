import subprocess
import shutil
from ..utils import print_command


def update_rust():
    if shutil.which("rustup"):
        print_command("rustup update")
        subprocess.run(["rustup", "update"], check=True)
        print()
        print()

    if shutil.which("cargo"):
        try:
            subprocess.run(
                ["cargo", "install-update", "--version"],
                check=True,
                capture_output=True,
            )
            print_command("cargo install-update -a")
            subprocess.run(["cargo", "install-update", "-a"], check=True)
            print()
            print()
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass


if __name__ == "__main__":
    update_rust()
