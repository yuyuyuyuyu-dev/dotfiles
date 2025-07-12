import os
import sys
import shutil

def create_symlink_safely(source_path, dest_path):
    """
    source_pathからdest_pathへのシンボリックリンクを安全に作成する。

    - リンク元が存在しない場合はエラー。
    - リンク先が正しいリンクなら何もしない。
    - リンク先が違うリンクなら貼り直す。
    - リンク先がファイルなら、内容を表示して削除・上書き確認後に処理。
    - リンク先がディレクトリなら、内容をすべて表示して再帰的に削除・上書き確認後に処理。
    """
    # 1. リンク元ファイルの存在チェック
    if not os.path.exists(source_path):
        print(f"[ERROR] Source file not found: {source_path}", file=sys.stderr)
        return

    # 2. リンク先に何かが存在する場合の処理
    if os.path.lexists(dest_path):
        # 既存のものがシンボリックリンクの場合
        if os.path.islink(dest_path):
            # 正しいリンクか確認
            if os.readlink(dest_path) == source_path:
                print(f"[SKIP] Symlink already exists and is correct: {dest_path}")
                return
            else:
                print(f"[INFO] Removing incorrect symlink: {dest_path}")
                os.remove(dest_path)
        # 既存のものがディレクトリの場合
        elif os.path.isdir(dest_path):
            print(f"[WARN] Directory already exists at destination: {dest_path}")
            print("--- Directory contents ---")
            try:
                contents = os.listdir(dest_path)
                for item in contents:
                    print(f"- {item}")
            except Exception as e:
                print(f"Could not list directory contents: {e}", file=sys.stderr)
            print("--------------------------")

            user_input = input(f"Do you want to remove it and create a symlink? [y/N]: ")
            if user_input.lower() == 'y':
                try:
                    shutil.rmtree(dest_path) # ディレクトリを再帰的に削除
                    print(f"[ACTION] Recursively deleted directory: {dest_path}")
                except OSError as e:
                    print(f"[ERROR] Could not delete directory {dest_path}: {e}. Skipping symlink creation.", file=sys.stderr)
                    return
            else:
                print("[SKIP] User declined to remove existing directory. Skipping symlink creation.")
                return
        # 既存のものがファイルの場合
        elif os.path.isfile(dest_path):
            print(f"[WARN] File already exists at destination: {dest_path}")
            print("--- Current content ---")
            try:
                with open(dest_path, 'r', encoding='utf-8') as f:
                    print(f.read())
            except Exception as e:
                print(f"Could not read file content: {e}", file=sys.stderr)
            print("-----------------------")

            user_input = input(f"Do you want to remove it and create a symlink? [y/N]: ")
            if user_input.lower() == 'y':
                print(f"[ACTION] Removing file: {dest_path}")
                os.remove(dest_path)
            else:
                print("[SKIP] User declined to remove existing file. Skipping.")
                return

    # 3. シンボリックリンクの作成
    print(f"[ACTION] Creating symlink: {dest_path} -> {source_path}")
    os.symlink(source_path, dest_path)

if __name__ == '__main__':
    # テスト用のコード
    # 実際の使用では、このモジュールをimportして関数を呼び出す
    print("This module is intended to be imported and used as a function.")
    print("Example: create_symlink_safely('/path/to/source', '/path/to/dest')")