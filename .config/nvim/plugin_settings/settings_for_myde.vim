" dockerコンテナ内向けの設定
" うまくいかなかったときの保険として、'name'の直の最初に'myde-'をつける
" この３つ以外は、パスが通ってるとこにコマンド配置しておけば勝手にいいかんじにしてくれる(vim-lsp-settingが)
if executable('/home/myde/omnisharp-lsp/run')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-omnisharp-lsp',
        \ 'cmd': {server_info->['/home/myde/omnisharp-lsp/run', '-lsp']},
        \ 'whitelist': ['cs']
        \ })
endif

if executable('/home/myde/eclipse-jdt-ls/eclipse-jdt-ls')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-eclipse-jdt-ls',
        \ 'cmd': {server_info->['/home/myde/eclipse-jdt-ls/eclipse-jdt-ls']},
        \ 'whitelist': ['java']
        \ })
endif

if executable('/home/myde/myCommands/kotlin-language-server')
  au User lsp_setup call lsp#register_server({
        \ 'name': 'myde-kotlin-language-server',
        \ 'cmd': {server_info->['/home/myde/myCommands/kotlin-language-server']},
        \ 'whitelist': ['kotlin']
        \ })
endif
