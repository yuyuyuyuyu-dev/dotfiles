
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""共通で利用するパス変数を管理するモジュール。

このスクリプト群で利用する、dotfilesのルートディレクトリや
ユーザーのホームディレクトリなどの基本的なパスを定数として提供します。
パスの計算ロジックを一元化し、再利用性を高めることを目的としています。
"""
import os

# このファイル (paths.py) の絶対パスを取得
_current_file_path = os.path.abspath(__file__)

# /deploy/shared ディレクトリのパス
_shared_dir = os.path.dirname(_current_file_path)

# /deploy ディレクトリのパス
_deploy_dir = os.path.dirname(_shared_dir)

# プロジェクトのルートディレクトリ (dotfiles) のパス
DOTFILES_ROOT = os.path.dirname(_deploy_dir)

# ユーザーのホームディレクトリのパス
HOME_DIR = os.path.expanduser('~')

# ~/.local/bin ディレクトリのパス
LOCAL_BIN_DIR = os.path.join(HOME_DIR, ".local", "bin")

# テスト用に、計算結果が正しいかを出力
if __name__ == '__main__':
    print(f"DOTFILES_ROOT: {DOTFILES_ROOT}")
    print(f"HOME_DIR: {HOME_DIR}")
    print(f"LOCAL_BIN_DIR: {LOCAL_BIN_DIR}")
