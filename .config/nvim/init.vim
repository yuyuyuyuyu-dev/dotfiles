" vimの設定を読み込む
if has('win64')
    source ~\_vimrc
else
    source ~/.vimrc
endif


" XDG Base Directory Specification
let s:config_home = has('win64') ? $LOCALAPPDATA : empty($XDG_CONFIG_HOME) ? $HOME . '/.config' : $XDG_CONFIG_HOME


" vim-plugがインストールされていなかったらインストールする
let s:plug_path = has('win64') ? $LOCALAPPDATA . '\nvim-data\site\autoload\plug.vim' : $HOME . '/.local/share/nvim/site/autoload/plug.vim'
if !filereadable(s:plug_path)
    call g:DownloadIfNotFileReadable(s:plug_path, 'https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')

    " ついでにプラグインもインストールする
    if has('win64')
        " Windowsの場合
        source ~/AppData/Local/nvim/plugin_settings/vim-plug_setting.vim
    else
        " Windows以外の場合
        if empty($XDG_CONFIG_HOME)
            source ~/.config/nvim/plugin_settings/vim-plug_setting.vim
        else
            source $XDG_CONFIG_HOME/nvim/plugin_settings/vim-plug_setting.vim
        endif
    endif
    PlugInstall
endif


" ここからneovim向けの設定
" カラースキームの設定
" gruvboxがローカルに無かったらダウンロードする
call g:DownloadIfNotFileReadable(has('win64') ? s:config_home . '\nvim\colors\gruvbox.vim' : s:config_home . '/nvim/colors/gruvbox.vim', 'https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')
" カラースキームをgruvboxに指定する
colorscheme gruvbox


" コマンドラインを２行にする
set cmdheight=2


" 開いたファイルがあるディレクトリに自動でcdする
set autochdir


" undo履歴を保持し続ける
if has('persistent_undo')
    set undodir=~/.vimundo
    set undofile
endif


" python3のパスを指定
if has('win64')
    if !empty(system('where /Q python && echo has'))
        let g:python3_host_prog = s:config_home . '\Programs\Python\Python37-32\python.exe'
    endif
else
    let g:python3_host_prog = system("bash -c 'echo -n $(which python3)'")
endif


" filetypeの追加
autocmd BufNewFile,BufRead *.fish setfiletype fish
autocmd BufNewFile,BufRead *.vim setfiletype vim
autocmd BufNewFile,BufRead *.swift setfiletype swift
autocmd BufNewFile,BufRead *.kt setfiletype kotlin


" プラグインの設定
" vim-plugの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-plug_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/vim-plug_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-plug_setting.vim
endif

" vim-indent-guidesの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-indent-guides_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/vim-indent-guides_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-indent-guides_setting.vim
endif

" asyncompleteの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/asyncomplete_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/asyncomplete_setting.vim
else
    source ~/.config/nvim/plugin_settings/asyncomplete_setting.vim
endif

" aleの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/ale_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/ale_setting.vim
else
    source ~/.config/nvim/plugin_settings/ale_setting.vim
endif

" vim-lspの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-lsp_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/vim-lsp_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-lsp_setting.vim
endif

" vim-closetagの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-closetag_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/nvim/plugin_settings/vim-closetag_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-closetag_setting.vim
endif

" myDE向けの設定を読み込む
if !empty($container_name)
    if !empty($XDG_CONFIG_HOME)
        source $SDG_CONFIG_HOME/nvim/plugin_settings/settings_for_myde.vim
    else
        source ~/.config/nvim/plugin_settings/settings_for_myde.vim
    endif
endif


" 補完の設定
imap <C-k> <Plug>(neosnippet_expand_or_jump)
smap <C-k> <Plug>(neosnippet_expand_or_jump)
xmap <C-k> <Plug>(neosnippet_expand_target)
if has('conceal')
    set conceallevel=0 concealcursor=niv
endif


" キーバインド
tnoremap <silent> <ESC> <C-\><C-n>
nnoremap <silent> <C-t><C-m> :split<CR> <C-w>j :terminal<CR> :resize 6<CR> i
nnoremap <silent> def :LspDefinition<CR>
