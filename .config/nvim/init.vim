" XDG Base Directory Specification
let s:config_home = empty($XDG_CONFIG_HOME) ? expand('~/.config') : $XDG_CONFIG_HOME


" vim-plugがインストールされていなかったらインストールする
if !filereadable(expand('~/.local/share/nvim/site/autoload/plug.vim'))
    call system('curl -fLo ~/.local/share/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')
endif


" カラースキームの"gruvbox"が無かったら用意する
if !filereadable(s:config_home . '/nvim/colors/gruvbox.vim')
    " ~/.vim/colorsの中にあったらリンクを貼る
    if filereadable(expand('~/.vim/colors/gruvbox.vim'))
        " $XDG_CONFIG_HOMEが設定されているかによってリンク先を変える
        if empty($XDG_CONFIG_HOME)
            call system('ln -fns ~/.vim/colors ~/.config/nvim/')
        else
            call system('ln -fns ~/.vim/colors $XDG_CONFIG_HOME/nvim/')
        endif
    else
        " ~/.vim/colorsの中にも無かったらダウンロードする
        " ダウンロード先のフォルダが無かったら作る
        if !isdirectory(s:config_home . '/nvim/colors')
            call mkdir(s:config_home . '/nvim/colors', 'p')
        endif

        " $XDG_CONFIG_HOMEが設定されているかによってダウンロード先を変える
        if empty($XDG_CONFIG_HOME)
            call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o ~/.config/nvim/colors/gruvbox.vim')
        else
            call system('curl https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim -o $XDG_CONFIG_HOME/nvim/colors/gruvbox.vim')
        endif
    endif
endif


" vimの設定を読み込む
source ~/.vimrc


" ここからneovim向けの設定
" コマンドラインを２行にする
set cmdheight=2


" 開いたファイルがあるディレクトリに自動でcdする
set autochdir


" シェルをfishに設定する
let s:os_type = system("bash -c 'echo -n $(uname)'")
if s:os_type ==# "Darwin"
    set shell=/usr/local/bin/fish
else
    set shell=/usr/bin/fish
endif


" python3のパスを指定
let g:python3_host_prog = system("bash -c 'echo -n $(which python3)'")


" filetypeの追加
autocmd BufNewFile,BufRead *.fish setfiletype fish
autocmd BufNewFile,BufRead *.vim setfiletype vim
autocmd BufNewFile,BufRead *.swift setfiletype swift
autocmd BufNewFile,BufRead *.kt setfiletype kotlin


" プラグインの設定
" vim-plugの設定を読み込む
source ~/.config/nvim/plugin_settings/vim-plug_setting.vim

" vim-indent-guidesの設定を読み込む
source ~/.config/nvim/plugin_settings/vim-indent-guides_setting.vim

" asyncompleteの設定を読み込む
source ~/.config/nvim/plugin_settings/asyncomplete_setting.vim

" aleの設定を読み込む
source ~/.config/nvim/plugin_settings/ale_setting.vim

" vim-lspの設定を読み込む
source ~/.config/nvim/plugin_settings/vim-lsp_setting.vim

" myDE向けの設定を読み込む
if !empty($container_name)
    source ~/.config/nvim/plugin_settings/settings_for_myde.vim
endif


" 補完の設定
imap <C-k> <Plug>(neosnippet_expand_or_jump)
smap <C-k> <Plug>(neosnippet_expand_or_jump)
xmap <C-k> <Plug>(neosnippet_expand_target)
if has('conceal')
    set conceallevel=2 concealcursor=niv
endif


" キーバインド
tnoremap <silent> <ESC> <C-\><C-n>
nnoremap <silent> <C-t><C-m> :split<CR> <C-w>j :terminal<CR> :resize 6<CR> i
nnoremap <silent> def :LspDefinition<CR>
