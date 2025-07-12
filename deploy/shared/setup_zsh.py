import os
from create_symlink_safely import create_symlink_safely
from paths import DOTFILES_ROOT, HOME_DIR

zsh_files = [
    '.zshrc',
    '.zprofile',
    '.zshenv',
    '.zlogout',
]

def setup_zsh():
    """
    ホームディレクトリにzsh関連ファイルのシンボリックリンクを安全に作成する
    """
    print("--- Setting up Zsh configurations ---")
    for zsh_file in zsh_files:
        source_path = os.path.join(DOTFILES_ROOT, zsh_file)
        dest_path = os.path.join(HOME_DIR, zsh_file)
        create_symlink_safely(source_path, dest_path)
    print("--- Zsh setup complete ---\n")

# このスクリプトが直接実行された場合のための処理
if __name__ == '__main__':
    setup_zsh()
