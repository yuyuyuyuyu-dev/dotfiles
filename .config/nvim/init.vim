" XDG Base Directory Specification
let s:config_home = has('win64') ? $LOCALAPPDATA : empty($XDG_CONFIG_HOME) ? expand('~/.config') : $XDG_CONFIG_HOME

let s:share_nvim_dir = has('win64') ? $LOCALAPPDATA . '/nvim-data' : $HOME . '/.local/share/nvim'


" シェルを指定する
let s:bash_path = system("bash -c 'echo -n $(which bash)'")
let s:fish_path = system("bash -c 'echo -n $(which fish)'")
" bash
if s:bash_path ==# "/usr/bin/bash"
    set shell=/usr/bin/bash
endif
" fish
" Linux
if s:fish_path ==# "/usr/bin/fish"
    set shell=/usr/bin/fish
endif
" Mac
if s:fish_path ==# "/usr/local/bin/fish"
    set shell=/usr/local/bin/fish
endif
" Linuxbrew
if s:fish_path ==# "/home/linuxbrew/.linuxbrew/bin/fish"
    set shell=/home/linuxbrew/.linuxbrew/bin/fish
endif
" Windows
if has('win64')
    set shell=C:\WINDOWS\system32\cmd.exe
endif


" vim-plugがインストールされていなかったらインストールする
if !filereadable(s:share_nvim_dir . '/site/autoload/plug.vim')
    let s:curl_command = has('win64') ? 'curl.exe -fLo ' . s:share_nvim_dir . '\site\autoload\plug.vim' : 'curl -fLo ' . s:share_nvim_dir . '/site/autoload/plug.vim'
    call system(s:curl_command . ' --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim')

    " ついでにプラグインもインストールする
    if has('win64')
        " Windowsの場合
        source ~/AppData/Local/nvim/plugin_settings/vim-plug_setting.vim
    else
        " Windows以外の場合
        source ~/.config/nvim/plugin_settings/vim-plug_setting.vim
    endif
    PlugInstall
endif


" カラースキームの"gruvbox"が無かったら用意する
if !filereadable(s:config_home . '/nvim/colors/gruvbox.vim')
    if filereadable($HOME . '/.vim/colors/gruvbox.vim')
        " ~/.vim/colorsの中にあったらリンクを貼る
        if !isdirectory(s:config_home . '/nvim/colors')
            call mkdir(s:config_home . '/nvim/colors', 'p')
        endif
        call system(has('win64') ? 'mklink ' . s:config_home . '\nvim\colors\gruvbox.vim %HOMEPATH%\.vim/colors/gruvbox.vim' : 'ln -fs ~/.vim/colors/gruvbox.vim ' . s:config_home . '/nvim/colors/gruvbox.vim')
    else
        " ~/.vim/colorsの中にも無かったらダウンロードする
        let s:curl_command = has('win64') ? 'curl.exe -fLo ' . s:config_home . '\nvim\colors\gruvbox.vim' : 'curl -o' . s:config_home . '/nvim/colors/gruvbox.vim'
        call system(s:curl_command . ' --create-dirs https://raw.githubusercontent.com/morhetz/gruvbox/master/colors/gruvbox.vim')
    endif
endif


" vimの設定を読み込む
if has('win64')
    source ~\_vimrc
else
    source ~/.vimrc
endif


" ここからneovim向けの設定
" コマンドラインを２行にする
set cmdheight=2


" 開いたファイルがあるディレクトリに自動でcdする
set autochdir


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
    source $XDG_CONFIG_HOME/.config/nvim/plugin_settings/vim-plug_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-plug_setting.vim
endif

" vim-indent-guidesの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-indent-guides_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/.config/nvim/plugin_settings/vim-indent-guides_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-indent-guides_setting.vim
endif

" asyncompleteの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/asyncomplete_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/.config/nvim/plugin_settings/asyncomplete_setting.vim
else
    source ~/.config/nvim/plugin_settings/asyncomplete_setting.vim
endif

" aleの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/ale_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/.config/nvim/plugin_settings/ale_setting.vim
else
    source ~/.config/nvim/plugin_settings/ale_setting.vim
endif

" vim-lspの設定を読み込む
if has('win64')
    source ~/AppData/Local/nvim/plugin_settings/vim-lsp_setting.vim
elseif !empty($XDG_CONFIG_HOME)
    source $XDG_CONFIG_HOME/.config/nvim/plugin_settings/vim-lsp_setting.vim
else
    source ~/.config/nvim/plugin_settings/vim-lsp_setting.vim
endif

" myDE向けの設定を読み込む
if !empty($container_name)
    if !empty($XDG_CONFIG_HOME)
        source $SDG_CONFIG_HOME/.config/nvim/plugin_settings/settings_for_myde.vim
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
