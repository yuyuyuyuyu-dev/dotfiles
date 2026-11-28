import os

from deploy.shared.create_symlink_safely import create_symlink_safely
from deploy.shared.paths import DOTFILES_ROOT, HOME_DIR


def setup_claude():
    print("--- Setting up Claude configurations ---")
    os.makedirs(os.path.join(HOME_DIR, ".claude"), exist_ok=True)
    for name in ("CLAUDE.md", "rules"):
        create_symlink_safely(
            os.path.join(DOTFILES_ROOT, ".claude", name),
            os.path.join(HOME_DIR, ".claude", name),
        )
    print("--- Claude setup complete ---\n")
