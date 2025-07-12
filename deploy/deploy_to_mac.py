#!/usr/bin/env python3

import sys
import os
import subprocess

# sharedディレクトリをPythonのモジュール検索パスに追加
sys.path.append(os.path.join(os.path.dirname(__file__), 'shared'))

# shared内のモジュールをインポート
from setup_bash import setup_bash
from setup_zsh import setup_zsh
from setup_git import setup_git
from setup_tmux import setup_tmux
from deploy_configs import deploy_configs
from setup_vim import setup_vim
from paths import HOME_DIR

def main():
    """macOS用のデプロイを実行する"""
    print("Starting deployment for macOS...")

    setup_bash()
    setup_zsh()

    setup_git()
    _setup_git_for_platform()

    setup_tmux()

    setup_vim()

    deploy_configs()

    print("\nmacOS deployment finished.")

def _setup_git_for_platform():
    """
    プラットフォーム固有のGit設定を行う。
    """
    print("--- Setting up platform specific Git configurations ---")

    # ~/.gitconfigが存在しなかったら作成する
    # このファイルが存在しなかった場合、`git config --global`で設定したときに~/.config/git/configに書き込まれてしまう
    gitconfig_path = os.path.join(HOME_DIR, '.gitconfig')
    if not os.path.exists(gitconfig_path):
        print(f"  Creating empty {gitconfig_path}")
        open(gitconfig_path, 'a').close()

    print("--- Platform specific Git setup complete ---\n")

if __name__ == "__main__":
    main()
