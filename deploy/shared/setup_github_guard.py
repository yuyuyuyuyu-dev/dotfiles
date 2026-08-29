import os
import shutil
import subprocess
import sys

from deploy.shared import paths

# The analyzer is installed from the remote default branch rather than from this
# checkout, so what guards the machine is a revision the tests have run against.
# The tests install from somewhere else, which is what these two are here for.
REPOSITORY = os.environ.get(
    "GITHUB_GUARD_REPOSITORY", "https://github.com/yuyuyuyuyu-dev/dotfiles.git"
)
BRANCH = os.environ.get("GITHUB_GUARD_BRANCH", "main")
CRATE = "github-guard"


def setup_github_guard():
    print("--- Setting up the GitHub guard ---")

    cargo = shutil.which("cargo")
    if cargo is None:
        print(f"[SKIP] cargo was not found, so {CRATE} was not installed.")
    else:
        print(f"[ACTION] Installing {CRATE} from {REPOSITORY} ({BRANCH})")
        # --locked builds the versions the lockfile pins. A failure is reported
        # rather than raised, so that a machine which cannot reach GitHub still
        # gets the rest of its dotfiles.
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

    print("--- GitHub guard setup complete ---\n")


def _report_registration():
    # The hook only runs once settings.json names it, and that file is left to be
    # edited by hand because Claude Code writes to it as well.
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
