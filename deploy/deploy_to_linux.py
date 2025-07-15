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
from deploy_configs import deploy_configs
from setup_vim import setup_vim
from setup_update_command import setup_update_command

def main():
    """Linux用のデプロイを実行する"""
    print("Starting deployment for Linux...")

    setup_bash()
    setup_zsh()

    setup_git()

    setup_vim()
    setup_update_command()

    deploy_configs()

    print("\nLinux deployment finished.")

if __name__ == "__main__":
    main()
