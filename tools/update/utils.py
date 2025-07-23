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
