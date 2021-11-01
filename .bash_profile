if type tmux > /dev/null 2>&1; then # tmuxがインストールされているかを確認
  if [ -z ${TMUX} ]; then # tmuxがすでに起動されていないか確認
    if tmux list-sessions > /dev/null 2>&1; then # すでにセッションが存在するか確認
      tmux -u attach && exit # 最新のセッションにアタッチ
    else tmux -u && exit # 新規セッションを作成
    fi
  fi
else
  echo 'tmuxがインストールされていません。'
fi

export PATH="$HOME/.cargo/bin:$PATH"
