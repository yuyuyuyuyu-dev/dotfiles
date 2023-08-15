#!/bin/bash

for i in .bash*; do
	# すでにファイルが存在した場合は元のファイルをリネームする
	if [ -e "${HOME}/${i}" ]; then
		mv -h "${HOME}/${i}" "${HOME}/${i}.bak"
	fi

	# リンクを貼る
	ln -fns "${HOME}/dotfiles/${i}" "${HOME}/${i}"
done
