import subprocess
from typing import Union


def run_command(
    command: Union[list[str], str],
    title: str,
    shell: bool = False,
    executable: str = None,
):
    print()
    print()
    print(f"##########")
    print(f"{title}")
    print(f"##########")

    command_str = command
    if isinstance(command, list):
        command_str = " ".join(command)
    print(f"$ {command_str}")

    subprocess.run(command, shell=shell, check=True, executable=executable)
