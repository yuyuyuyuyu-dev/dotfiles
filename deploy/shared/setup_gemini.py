import os

from deploy.shared import paths
from deploy.shared.create_symlink_safely import create_symlink_safely


def setup_gemini():
    print("--- Setting up Gemini ---")

    dotfiles_gemini_dir = os.path.join(paths.DOTFILES_ROOT, ".gemini")
    source_path = os.path.join(dotfiles_gemini_dir, "GEMINI.md")

    home_gemini_dir = os.path.join(paths.HOME_DIR, ".gemini")
    destination_path = os.path.join(home_gemini_dir, "GEMINI.md")

    os.makedirs(home_gemini_dir, exist_ok=True)

    create_symlink_safely(source_path, destination_path)

    print("--- Gemini setup complete ---\n")
