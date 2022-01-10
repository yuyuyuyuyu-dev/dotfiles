# ログインシェルのときに読み込まれるファイル
# (.zshrcより先に読み込まれる)
# ログインシェルのときだけ必要な設定はここに書く


case "$OSTYPE" in
    darwin*)
        # darwin(≒Mac)のときに読み込まれる設定
        ;;
    linux*)
        # linuxのときに読み込まれる設定
        ;;
esac

eval "$(/opt/homebrew/bin/brew shellenv)"
