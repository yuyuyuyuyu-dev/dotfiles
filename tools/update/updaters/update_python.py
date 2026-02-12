import subprocess
import shutil
import sysconfig
import os
from ..utils import print_command


def update_python():
    errors = []
    if shutil.which("pyenv"):
        try:
            subprocess.run(
                ["pyenv", "update", "--help"], check=True, capture_output=True
            )
            cmd = "pyenv update"
            print_command(cmd)
            try:
                subprocess.run(["pyenv", "update"], check=True)
            except subprocess.CalledProcessError:
                errors.append(cmd)
            print()
            print()
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    if shutil.which("pip3"):
        if _is_externally_managed():
            print_command("pip3 self-update (skipped - externally managed)")
            print("Python environment is externally managed. Use your package manager to update pip.")
        else:
            cmd = "pip3 install -U pip --user"
            print_command(cmd)
            try:
                subprocess.run(["pip3", "install", "-U", "pip", "--user"], check=True)
            except subprocess.CalledProcessError:
                errors.append(cmd)
        print()
        print()

    return errors


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
