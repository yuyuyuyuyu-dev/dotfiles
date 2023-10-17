@echo off


rem シンボリックリンクを貼る
mklink %homepath%\_vimrc %homepath%\dotfiles\.vimrc
mklink /D %homepath%\.config %homepath%\dotfiles\.config
mklink /D %localappdata%\nvim\lua\custom %homepath%\dotfiles\nvchad\custom
mklink %appdata%\Hyper\.hyper.js %homepath%\dotfiles\.hyper.js
mklink %homepath%\.nyagos %homepath%\dotfiles\.nyagos


rem Gitの設定をする
rem %homepath%\.gitconfigが存在しなかったら作成する
rem このファイルが存在しなかった場合、`git config --global`で設定したときに%homepath%\.config\git\configに書き込まれてしまう
if not exist %homepath%\.gitconfig type nul >> %homepath%\.gitconfig

rem 改行コード
rem プッシュするときはLFに変換する
rem プルするときはCRLFに変換する
git config --global core.autocrlf true

rem SSHの公開鍵を指定する
git config --global user.signingkey %homepath%\.ssh\id_ed25519.pub


pause
