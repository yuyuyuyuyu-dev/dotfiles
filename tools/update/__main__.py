"""
コマンドラインから使用するあらゆるツールの更新処理をまとめる汎用コマンド。

このスクリプトは、OSのパッケージマネージャー(Homebrew, APT)から、
言語固有のツールチェイン(rustup, Volta)、パッケージ管理(npm, Cargo)まで、
実行環境に存在するツールのアップデートを網羅的に実行することを目的とする。

各更新処理は `updaters` ディレクトリ内のモジュールとして実装され、
この関数から順次呼び出される。
"""

from .updaters.update_apt import update_apt
from .updaters.update_brew import update_brew
from .updaters.update_npm import update_npm
from .updaters.update_python import update_python
from .updaters.update_rust import update_rust
from .updaters.update_sdkman import update_sdkman
from .updaters.update_volta import update_volta
from .utils import print_result


def main():
    errors = []
    errors.extend(update_brew())
    errors.extend(update_apt())
    errors.extend(update_sdkman())
    errors.extend(update_volta())
    errors.extend(update_npm())
    errors.extend(update_rust())
    errors.extend(update_python())
    print_result(errors)


if __name__ == "__main__":
    main()
