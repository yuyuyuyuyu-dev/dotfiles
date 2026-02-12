def print_command(command):
    """
    コマンドを囲んで出力します。
    """
    command_with_quotes = f"`{command}`"
    line = "#" * (len(command_with_quotes) + 4)
    empty_line = f"#{' ' * (len(command_with_quotes) + 2)}#"
    print()
    print(line)
    print(empty_line)
    print(f"# {command_with_quotes} #")
    print(empty_line)
    print(line)
    print()


def print_result(errors):
    """
    更新結果を表示します。
    """
    if not errors:
        print("Success")
    else:
        print("Error:")
        for error in errors:
            print(f"- `{error}`")
