#!/usr/bin/env fish

# ${HOME}/dotfiles内にある.(ドット)から始まるファイル(${EXCEPTION_ARRAY}に追加したものは除く)のシンボリックリンクを${HOME}ディレクトリ内に貼る

function main
  # 「.」から始まるファイルのリンクを貼る
  for file in .*
    # リンクを貼りたくないファイルは無視する
    if ignore {$file}
      continue
    end

    # リンク先にファイルが存在していたらリネームする
    if test -e {$HOME}/{$file}
      mv {$HOME}/{$file} {$HOME}/{$file}.bak
    end

    # リンクを貼る
    ln -fns {$HOME}/dotfiles/{$file} {$HOME}/{$file}
  end


  # NvChadの設定ファイル
  if test -e {$HOME}/.config/nvim/lua/custom
    mv {$HOME}/.config/nvim/lua/custom {$HOME}/.config/nvim/lua/custom.bak
  end
  ln -fns {$HOME}/dotfiles/nvchad/custom {$HOME}/.config/nvim/lua/custom


  # Gitの設定をする
  set-git-config
end


# リンクを貼るかどうかを判定する関数
function ignore
  # リンクを貼りたくないものはここに入れておく
  set ignores '.' '..' '.DS_Store' '.git' '.gitignore' '.hyper.js'

  for i in {$ignores}
    if test {$i} = {$argv}
      return 0
    end
  end
  return 1
end


# Gitの設定をする関数
function set-git-config
  # デフォルトブランチの名前を"main"に指定する
  git config --global init.defaultBranch main

  # ユーザー名が設定されてなかったら対話形式で設定する
  if ! git config --global user.name > /dev/null 2>&1
    read -p'echo "ユーザー名に設定する文字列を入力してください> "' name
    git config --global user.name "$name"
  end
  # メールアドレスが設定されてなかったら対話形式で設定する
  if ! git config --global user.email > /dev/null 2>&1
    read -p'echo "ユーザーのメールアドレスに設定する文字列を入力してください> "' email
    git config --global user.email "$email"
  end

  # `git root` でリポジトリのルートディレクトリを出力する
  git config --global alias.root 'rev-parse --show-toplevel'

  # マージするときにFast-forwordを行わない
  git config --global merge.ff false

  # プッシュするときの認証に使う公開鍵を指定する
  git config --global user.signingkey ~/.ssh/id_ed25519.pub

  # 文字コード
  # git diffしたときの文字コードをutf-8にする
  git config --global core.pager 'LESSCHARSET=utf-8 less -cmN'

  # 改行コード
  # プッシュするときに改行コードをLFで揃える
  # プルするときは変換しない
  git config --global core.autocrlf input
  # git diffしたときに、改行コードを気にしないようにする
  git config --global alias.diff 'diff -w'

  # 常に読み込まれるgitignoreファイルを指定する
  git config --global core.excludesFile ~/.config/git/ignore

  # プルしたときの設定
  git config --global pull.rebase true
  git config --global pull.ff true

  # コミットに署名をする
  # 署名を有効化
  git config --global commit.gpgsign true
  # ssh公開鍵で署名する
  git config --global gpg.format ssh

  # フェッチとプルしたときにリモートブランチの削除状況に追従する
  git config --global alias.fetch 'fetch -p'
  git config --global alias.pull 'pull -p'
end


main
