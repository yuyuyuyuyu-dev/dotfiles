import subprocess
import shutil
from ..utils import print_command


def update_claude():
    errors = []
    if not shutil.which("claude"):
        return errors

    cmd = "claude update"
    print_command(cmd)
    try:
        subprocess.run(["claude", "update"], check=True)
    except subprocess.CalledProcessError:
        errors.append(cmd)
    print()
    print()

    return errors
