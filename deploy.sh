#!/data/user/0/com.droidvim/files/usr/bin/bash

for i in .bash*; do
	# すでにファイルが存在した場合は元のファイルをリネームする
	if [ -e "${HOME}/${i}" ]; then
		mv -h "${HOME}/${i}" "${HOME}/${i}.bak"
	fi

	# リンクを貼る
	ln -fns "${HOME}/dotfiles/${i}" "${HOME}/${i}"
done

# Gitの設定
# デフォルトブランチの名前を"main"に指定する
git config --global init.defaultBranch main

# プッシュするときの認証に使う公開鍵を指定する
git config --global user.signingkey ~/.ssh/id_ed25519.pub

# プッシュするときに改行コードをLFで揃える
# プルするときは変換しない
git config --global core.autocrlf input

# git diffしたときの文字コードをutf-8にする
git config --global core.pager 'LESSCHARSET=utf-8 less -cmN'

# git diffしたときに、改行コードを気にしないようにする
git config --global alias.diff 'diff -w'

# 常に読み込まれるgitignoreファイルを指定する
git config --global core.excludesFile ~/.config/git/ignore

# `git root` でリポジトリのルートディレクトリを出力する
git config --global alias.root 'rev-parse --show-toplevel'

# プルしたときの設定
git config --global pull.rebase true
git config --global pull.ff true

# コミットに署名をする
# 署名を有効化
git config --global commit.gpgsign true
# ssh公開鍵で署名する
git config --global gpg.format ssh
