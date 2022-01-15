#!/bin/bash

# ${HOME}/dotfiles内にある.(ドット)から始まるファイル(${EXCEPTION_ARRAY}に追加したものは除く)のシンボリックリンクを${HOME}ディレクトリ内に貼る


# リンクを貼りたくないものをここに入れておく
readonly EXCEPTION_ARRAY=('.' '..' '.DS_Store' '.git' '.gitignore')

for i in .*; do
  for j in ${EXCEPTION_ARRAY[@]}; do
    if [ ${i} = ${j} ]; then
      continue 2
    fi
  done

  # もともとファイルがあるとリンクを貼れないから削除
  if [ -e ${HOME}/${i} ]; then
    rm -rf ${HOME}/${i}
  fi

  ln -fns ${HOME}/dotfiles/${i} ${HOME}/
done


# 端末固有のgit設定
readonly GITCONFIGLOCAL='gitconfig.local'
if ! [ -e ${HOME}/.${GITCONFIGLOCAL} ]; then
    cp ${GITCONFIGLOCAL}.template ${HOME}/.${GITCONFIGLOCAL}
fi
