import os
from paths import DOTFILES_ROOT, HOME_DIR
from create_symlink_safely import create_symlink_safely

def setup_vim():
    """
    Vimの設定ファイルをセットアップする。
    .vimrc のシンボリックリンクをホームディレクトリに作成する。
    """
    print("--- Setting up Vim configuration ---")

    source_path = os.path.join(DOTFILES_ROOT, '.vimrc')
    dest_path = os.path.join(HOME_DIR, '.vimrc')

    create_symlink_safely(source_path, dest_path)

    print("--- Vim setup complete ---\n")

if __name__ == '__main__':
    setup_vim()
