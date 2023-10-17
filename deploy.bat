@echo off

mklink %homepath%\_vimrc %homepath%\dotfiles\.vimrc
mklink /D %localappdata%\nvim\lua\custom %homepath%\dotfiles\nvchad\custom
mklink %appdata%\Hyper\.hyper.js %homepath%\dotfiles\.hyper.js
mklink %homepath%\.nyagos %homepath%\dotfiles\.nyagos

rem Gitの設定
rem デフォルトブランチの名前を"main"に指定する
git config --global init.defaultBranch main

rem `git root` でリポジトリのルートディレクトリを出力する
git config --global --replace-all alias.root "rev-parse --show-toplevel"

rem マージするときにFast-forwordを行わない
git config --global merge.ff false

rem プッシュするときの認証に使う公開鍵を指定する
git config --global user.signingkey ~/.ssh/id_ed25519.pub

rem 文字コード
rem git diffしたときの文字コードをutf-8にする
git config --global core.pager "LESSCHARSET=utf-8 less -cmN"

rem 改行コード
rem プッシュするときに改行コードをLFで揃える
rem プルするときは変換しない
git config --global core.autocrlf input
rem git diffしたときに、改行コードを気にしないようにする
git config --global --replace-all alias.diff "diff -w"

rem 常に読み込まれるgitignoreファイルを指定する
git config --global core.excludesFile ~/.config/git/ignore

rem プルしたときの設定
git config --global pull.rebase true
git config --global pull.ff true

rem コミットに署名をする
rem 署名を有効化
git config --global commit.gpgsign true
rem ssh公開鍵で署名する
git config --global gpg.format ssh

git config --global --replace-all alias.pull "pull -p"
git config --global --replace-all alias.fetch "fetch -p"

pause
