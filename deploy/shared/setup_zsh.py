import os

from deploy.shared.create_symlink_safely import create_symlink_safely
from deploy.shared.paths import DOTFILES_ROOT, HOME_DIR

zsh_files = [
    ".zshrc",
    ".zprofile",
    ".zshenv",
    ".zlogout",
]


def setup_zsh():
    print("--- Setting up Zsh configurations ---")
    for zsh_file in zsh_files:
        source_path = os.path.join(DOTFILES_ROOT, zsh_file)
        dest_path = os.path.join(HOME_DIR, zsh_file)
        create_symlink_safely(source_path, dest_path)
    print("--- Zsh setup complete ---\n")


if __name__ == "__main__":
    setup_zsh()
