import subprocess
import shutil
from ..utils import print_command


def update_rust():
    errors = []
    if shutil.which("rustup"):
        cmd = "rustup update"
        print_command(cmd)
        try:
            subprocess.run(["rustup", "update"], check=True)
        except subprocess.CalledProcessError:
            errors.append(cmd)
        print()
        print()

    if shutil.which("cargo"):
        try:
            subprocess.run(
                ["cargo", "install-update", "--version"],
                check=True,
                capture_output=True,
            )
            cmd = "cargo install-update -a"
            print_command(cmd)
            try:
                subprocess.run(["cargo", "install-update", "-a"], check=True)
            except subprocess.CalledProcessError:
                errors.append(cmd)
            print()
            print()
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    return errors


if __name__ == "__main__":
    update_rust()
