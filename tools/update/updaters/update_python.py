import subprocess
import shutil
import sysconfig
import os
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
        if _is_externally_managed():
            print_command("pip3 self-update (skipped - externally managed)")
            print("Python environment is externally managed. Use your package manager to update pip.")
        else:
            print_command("pip3 install -U pip --user")
            subprocess.run(["pip3", "install", "-U", "pip", "--user"], check=True)
        print()
        print()


def _is_externally_managed():
    """Check if Python environment is externally managed by testing pip behavior."""
    try:
        result = subprocess.run(
            ["pip3", "install", "--dry-run", "--upgrade", "pip"],
            capture_output=True, text=True, timeout=10
        )
        return "externally-managed-environment" in result.stderr
    except Exception:
        return False


if __name__ == "__main__":
    update_python()
