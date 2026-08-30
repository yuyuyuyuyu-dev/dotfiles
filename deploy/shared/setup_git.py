import os
import subprocess

from deploy.shared.paths import HOME_DIR


def setup_git():
    print("--- Setting up common Git configurations ---")

    gitconfig_path = os.path.join(HOME_DIR, ".gitconfig")
    if not os.path.exists(gitconfig_path):
        print(f"  Creating empty {gitconfig_path}")
        open(gitconfig_path, "a").close()

    subprocess.run(
        ["git", "config", "--global", "core.pager", "LESSCHARSET=utf-8 less -cmN"],
        check=True,
    )
    print("  Set git config core.pager.")

    subprocess.run(["git", "config", "--global", "core.autocrlf", "input"], check=True)
    print("  Set git config core.autocrlf.")

    print("--- Common Git setup complete ---\n")


if __name__ == "__main__":
    setup_git()
