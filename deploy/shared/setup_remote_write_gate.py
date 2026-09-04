import os
import shutil
import subprocess
import sys

from deploy.shared import paths

REPOSITORY = "https://github.com/yuyuyuyuyu-dev/dotfiles.git"
BRANCH = "main"
CRATE = "remote-write-gate"


def setup_remote_write_gate():
    print("--- Setting up the remote write gate ---")

    cargo = shutil.which("cargo")
    if cargo is None:
        print(f"[SKIP] cargo was not found, so {CRATE} was not installed.")
    else:
        print(f"[ACTION] Installing {CRATE} from {REPOSITORY} ({BRANCH})")
        result = subprocess.run(
            [
                cargo,
                "install",
                "--git",
                REPOSITORY,
                "--branch",
                BRANCH,
                "--locked",
                CRATE,
            ],
            check=False,
        )
        if result.returncode != 0:
            print(
                f"[ERROR] cargo install failed, so {CRATE} is missing or out of date.",
                file=sys.stderr,
            )

    _report_registration()

    print("--- Remote write gate setup complete ---\n")


def _report_registration():
    settings_path = os.path.join(paths.HOME_DIR, ".claude", "settings.json")
    try:
        with open(settings_path, encoding="utf-8") as handle:
            settings = handle.read()
    except OSError:
        settings = ""

    if CRATE in settings:
        return

    print(f"[TODO] {settings_path} does not mention {CRATE} yet, so the hook is idle.")
    print("       Ask Claude Code to register it, for example:")
    print(f'       "Register ~/.cargo/bin/{CRATE} as a PreToolUse hook for Bash"')
