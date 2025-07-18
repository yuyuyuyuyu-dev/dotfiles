import os
from deploy.shared.create_symlink_safely import create_symlink_safely
from deploy.shared.paths import DOTFILES_ROOT, HOME_DIR

bash_files = [
    ".bash_profile",
    ".bashrc",
]


def setup_bash():
    """
    ホームディレクトリにbash関連ファイルのシンボリックリンクを安全に作成する
    """
    print("--- Setting up Bash configurations ---")
    for bash_file in bash_files:
        source_path = os.path.join(DOTFILES_ROOT, bash_file)
        dest_path = os.path.join(HOME_DIR, bash_file)
        create_symlink_safely(source_path, dest_path)
    print("--- Bash setup complete ---\n")


if __name__ == "__main__":
    setup_bash()
