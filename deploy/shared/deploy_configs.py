import os
from create_symlink_safely import create_symlink_safely
from paths import DOTFILES_ROOT, HOME_DIR

def deploy_configs():
    """
    dotfiles/.config/ ディレクトリ全体を ~/.config/ へシンボリックリンクする。
    """
    print("--- Deploying .config/ directory symlink ---")

    source_path = os.path.join(DOTFILES_ROOT, '.config')
    dest_path = os.path.join(HOME_DIR, '.config')

    create_symlink_safely(source_path, dest_path)

    print("--- .config/ directory deployment complete ---\n")

if __name__ == '__main__':
    deploy_configs()