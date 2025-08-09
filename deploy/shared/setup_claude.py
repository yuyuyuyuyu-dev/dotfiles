import os
from shared.create_symlink_safely import create_symlink_safely
from .paths import DOTFILES_ROOT, HOME_DIR


def setup_claude():
    """
    Set up .claude/CLAUDE.md symlink.
    """
    source_path = os.path.join(DOTFILES_ROOT, ".claude", "CLAUDE.md")
    dest_path = os.path.join(HOME_DIR, ".claude", "CLAUDE.md")

    print("--- Setting up Claude configurations ---")
    os.makedirs(os.path.dirname(dest_path), exist_ok=True)
    create_symlink_safely(source_path, dest_path)
    print("--- Claude setup complete ---\n")
