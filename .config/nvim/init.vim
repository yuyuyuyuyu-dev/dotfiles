" dein.vimが無かったらインストール
let s:cache_home    = empty($XDG_CACHE_HOME) ? expand('~/.cache') : $XDG_CACHE_HOME
let s:dein_dir      = s:cache_home . '/dein'
let s:dein_repo_dir = s:dein_dir . '/repos/github.com/Shougo/dein.vim'

if !isdirectory(s:dein_repo_dir)
  call system('git clone https://github.com/Shougo/dein.vim ' . shellescape(s:dein_repo_dir))
endif


" deinを使うために必要な設定
if &compatible
  set nocompatible               " Be iMproved
endif

" Required:
let &runtimepath = &runtimepath .",". s:dein_repo_dir

" Required:
if dein#load_state(s:dein_dir)
  call dein#begin(s:dein_dir)

  " Let dein manage dein
  " Required:
  call dein#add(s:dein_repo_dir)

  " Add or remove your plugins here:
  let s:config_home = empty($XDG_CONFIG_HOME) ? expand('~/.config') : $XDG_CONFIG_HOME
  let s:toml_dir    = s:config_home . '/nvim/toml' 
  let s:toml        = s:toml_dir . '/dein.toml'
  let s:lazy_toml   = s:toml_dir . '/dein_lazy.toml'
  let s:all_toml    = s:toml_dir . '/dein_all.toml'

  call dein#load_toml(s:toml,      {'lazy': 0})
  call dein#load_toml(s:lazy_toml, {'lazy': 1})

  " call dein#load_toml(s:all_toml,  {'lazy': 0})

  "call dein#add('Shougo/neosnippet.vim')
  "call dein#add('Shougo/neosnippet-snippets')

  " You can specify revision/branch/tag.
  "call dein#add('Shougo/deol.nvim', { 'rev': '01203d4c9' })

  " Required:
  call dein#end()
  call dein#save_state()
endif

" Required:
filetype plugin indent on
syntax enable

" If you want to install not installed plugins on startup.
if dein#check_install()
  call dein#install()
endif


"deopleteとneosnippetの競合を回避するための設定
imap <C-k> <Plug>(neosnippet_expand_or_jump)
smap <C-k> <Plug>(neosnippet_expand_or_jump)
xmap <C-k> <Plug>(neosnippet_expand_target)
if has('conceal')
	set conceallevel=2 concealcursor=niv
endif


" vimの設定
source ~/.vimrc


" neovimの設定
" python3のパスを指定
let g:python3_host_prog = system('(echo -n $(which python3))')

" filetypeの追加
autocmd BufNewFile,BufRead *.fish setfiletype fish
autocmd BufNewFile,BufRead *.vim setfiletype vim
autocmd BufNewFile,BufRead *.swift setfiletype swift
autocmd BufNewFile,BufRead *.kt setfiletype kotlin


" メモ
" pythonのneovimが見つからなくてdeopleteとかがエラーになっていたときは、pipでインストールした
" neovim, greenlet, pynvim, mspackgesをインストールし直したら治った
