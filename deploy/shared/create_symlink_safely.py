import os
import shutil
import sys


def create_symlink_safely(source_path, dest_path):
    if not os.path.exists(source_path):
        print(f"[ERROR] Source file not found: {source_path}", file=sys.stderr)
        return

    if os.path.lexists(dest_path):
        if os.path.islink(dest_path):
            if os.readlink(dest_path) == source_path:
                print(f"[SKIP] Symlink already exists and is correct: {dest_path}")
                return
            else:
                print(f"[INFO] Removing incorrect symlink: {dest_path}")
                os.remove(dest_path)
        elif os.path.isdir(dest_path):
            print(f"[WARN] Directory already exists at destination: {dest_path}")
            print("--- Directory contents ---")
            try:
                contents = os.listdir(dest_path)
                for item in contents:
                    print(f"- {item}")
            except OSError as e:
                print(f"Could not list directory contents: {e}", file=sys.stderr)
            print("--------------------------")

            user_input = input("Do you want to remove it and create a symlink? [y/N]: ")
            if user_input.lower() == "y":
                try:
                    shutil.rmtree(dest_path)
                    print(f"[ACTION] Recursively deleted directory: {dest_path}")
                except OSError as e:
                    print(
                        f"[ERROR] Could not delete directory {dest_path}: {e}. Skipping symlink creation.",
                        file=sys.stderr,
                    )
                    return
            else:
                print(
                    "[SKIP] User declined to remove existing directory. Skipping symlink creation."
                )
                return
        elif os.path.isfile(dest_path):
            print(f"[WARN] File already exists at destination: {dest_path}")
            print("--- Current content ---")
            try:
                with open(dest_path, encoding="utf-8") as f:
                    print(f.read())
            except (OSError, UnicodeDecodeError) as e:
                print(f"Could not read file content: {e}", file=sys.stderr)
            print("-----------------------")

            user_input = input("Do you want to remove it and create a symlink? [y/N]: ")
            if user_input.lower() == "y":
                print(f"[ACTION] Removing file: {dest_path}")
                os.remove(dest_path)
            else:
                print("[SKIP] User declined to remove existing file. Skipping.")
                return

    print(f"[ACTION] Creating symlink: {dest_path} -> {source_path}")
    os.symlink(source_path, dest_path)


if __name__ == "__main__":
    print("This module is intended to be imported and used as a function.")
    print("Example: create_symlink_safely('/path/to/source', '/path/to/dest')")
