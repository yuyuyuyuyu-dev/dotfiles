#!/usr/bin/env fish

# ${HOME}/dotfiles内にある.(ドット)から始まるファイル(${EXCEPTION_ARRAY}に追加したものは除く)のシンボリックリンクを${HOME}ディレクトリ内に貼る


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


# 「.」から始まるファイルのリンクを貼る
for file in .*
    # リンクを貼りたくないファイルは無視する
    if ignore {$file}
        continue
    end

    # リンク先にファイルが存在していたらリネームする
    if test -e {$HOME}/{$file}
        mv -h {$HOME}/{$file} {$HOME}/{$file}.bak
    end

    # リンクを貼る
    ln -fns {$HOME}/dotfiles/{$file} {$HOME}/{$file}
end

# NvChadの設定ファイル
if test -e {$HOME}/.config/nvim/lua/custom
    mv -h {$HOME}/.config/nvim/lua/custom {$HOME}/.config/nvim/lua/custom.bak
end
ln -fns {$HOME}/dotfiles/nvchad/custom {$HOME}/.config/nvim/lua/custom

# gitの設定ファイル（端末固有なもの用）
if ! test -e {$HOME}/gitconfig.local
    cp gitconfig.local.template {$HOME}/.gitconfig.local
end
