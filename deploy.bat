@echo off
setlocal
setlocal enabledelayedexpansion


mklink %homepath%\_vimrc %homepath%\dotfiles\.vimrc
mklink /D %homepath%\.config %homepath%\dotfiles\.config
mklink /D %localappdata%\nvim\lua %homepath%\dotfiles\nvchad\lua
mklink %homepath%\.nyagos %homepath%\dotfiles\.nyagos


copy %homepath%\dotfiles\hyper.js %appdata%\Hyper\.hyper.js
echo module.exports.config.shell = 'C:\\WINDOWS\\system32\\wsl.exe' >> %appdata%\Hyper\.hyper.js
echo module.exports.config.shellArgs = ['~'] >> %appdata%\Hyper\.hyper.js


if not exist %homepath%\.gitconfig type nul >> %homepath%\.gitconfig

git config --global user.name > nul 2>&1
if errorlevel 1 (
  set /p name="ユーザー名に設定する文字列を入力してください> "
  git config --global user.name !name!
)
git config --global user.email > nul 2>&1
if errorlevel 1 (
  set /p email="ユーザーのメールアドレスに設定する文字列を入力してください> "
  git config --global user.email !email!
)

git config --global core.autocrlf true

git config --global merge.ff true

git config --global user.signingkey %homedrive%%homepath%\.ssh\id_ed25519.pub


pause
