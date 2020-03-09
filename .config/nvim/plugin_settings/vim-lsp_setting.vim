" コンテナ内だとうまくパスが通せてなかったりするからmyCommandsフォルダに入れてあるプログラムは絶対パスで指定する
" うまくいかなかったときの保険として、'name'の直の最初に'myde-'をつける
if executable('pyls')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-pyls',
        \ 'cmd': {server_info->['pyls']},
        \ 'whitelist': ['python']
        \ })
endif

if executable('bash-language-server')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-bash-language-server',
        \ 'cmd': {server_info->['bash-language-server', 'start']},
        \ 'whitelist': ['sh']
        \ })
endif

" windowsのdockerで動かしたらなぜか動かなかったからパスを直で書く
" そのうち修正したい
" let s:omnisharp_lsp = expand('~/omnisharp-lsp/run')
if executable('/home/myde/omnisharp-lsp/run')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-omnisharp-lsp',
        \ 'cmd': {server_info->['/home/myde/omnisharp-lsp/run', '-lsp']},
        \ 'whitelist': ['cs']
        \ })
endif

if executable('clangd-8')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-clangd',
        \ 'cmd': {server_info->['clangd-8']},
        \ 'whitelist': ['c', 'cpp', 'objc', 'objcpp']
        \ })
endif

if executable('typescript-language-server')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-typescript-language-server',
        \ 'cmd': {server_info->['typescript-language-server', '--stdio']},
        \ 'whitelist': ['javascript', 'javascriptreact', 'typescript', 'typescriptreact', 'typescript.tsx']
        \ })
endif

" なんかうまく動かないからコメントアウト
" if executable('/home/myde/myCommands/eclipse-jdt-ls')
"   au User lsp_setup call lsp#register_server({
"         \ 'name': 'myde-eclipse-jdt-ls',
"         \ 'cmd': {server_info->['/home/myde/myCommands/eclipse-jdt-ls']},
"         \ 'whitelist': ['java']
"         \ })
" endif

if executable('vim-language-server')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-vim-language-server',
        \ 'cmd': {server_info->['vim-language-server', '--stdio']},
        \ 'whitelist': ['vim']
        \ })
endif

if executable('docker-langserver')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-docker-langserver',
        \ 'cmd': {server_info->['docker-langserver', '--stdio']},
        \ 'whitelist': ['dockerfile']
        \ })
endif

if executable('json-languageserver')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-json-languageserver',
        \ 'cmd': {server_info->['json-languageserver', '--stdio']},
        \ 'whitelist': ['json', 'jsonc']
        \ })
endif

if executable('/home/myde/myCommands/kotlin-language-server')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-kotlin-language-server',
        \ 'cmd': {server_info->['/home/myde/myCommands/kotlin-language-server']},
        \ 'whitelist': ['kotlin']
        \ })
endif
