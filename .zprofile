# ログインシェルのときに読み込まれるファイル
# (.zshrcより先に読み込まれる)
# ログインシェルのときだけ必要な設定はここに書く


if [ -z $TMUX ]; then
  if tmux list-sessions; then
    tmux -u attach && exit
  else
    tmux -u && exit
  fi
fi

case "$OSTYPE" in
  darwin*)
    # darwin(≒Mac)のときに読み込まれる設定
    ;;
  linux*)
    # linuxのときに読み込まれる設定
    ;;
esac
