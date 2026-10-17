import os
import shutil
import subprocess
import sys

from deploy.shared import paths

REPOSITORY = "https://github.com/yuyuyuyuyu-dev/dotfiles.git"
HOOK = "claude-gh-admission-hook"
KOTLINSTALL_BIN = os.path.join(paths.HOME_DIR, ".kotlinstall", "bin")


def setup_claude_gh_admission_hook():
    print("--- Setting up the admission hook ---")

    kotlinstall = shutil.which("kotlinstall") or shutil.which(
        "kotlinstall", path=KOTLINSTALL_BIN
    )
    if kotlinstall is None:
        print(f"[SKIP] kotlinstall was not found, so {HOOK} was not installed.")
    else:
        print(f"[ACTION] Installing {HOOK} from {REPOSITORY}")
        result = subprocess.run(
            [
                kotlinstall,
                "install",
                REPOSITORY,
                f"--subdirectory={HOOK}",
            ],
            check=False,
        )
        if result.returncode != 0:
            print(
                f"[ERROR] kotlinstall install failed, so {HOOK} is missing or out of date.",
                file=sys.stderr,
            )

    _report_registration()

    print("--- Admission hook setup complete ---\n")


def _report_registration():
    settings_path = os.path.join(paths.HOME_DIR, ".claude", "settings.json")
    try:
        with open(settings_path, encoding="utf-8") as handle:
            settings = handle.read()
    except OSError:
        settings = ""

    if HOOK in settings:
        return

    print(f"[TODO] {settings_path} does not mention {HOOK} yet, so the hook is idle.")
    print("       Ask Claude Code to register it, for example:")
    print(
        f'       "Register ~/.kotlinstall/bin/{HOOK} as a PreToolUse hook for every tool"'
    )
