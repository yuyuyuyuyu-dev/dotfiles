" ノーマルモードで"def"と入力したら変数や関数の定義元にジャンプするようにする
nnoremap <silent> def :LspDefinition<CR>

" "neosnippet"からの補完情報を取得するようにする
call asyncomplete#register_source(asyncomplete#sources#neosnippet#get_source_options({
    \ 'name': 'neosnippet',
    \ 'whitelist': ['*'],
    \ 'completor': function('asyncomplete#sources#neosnippet#completor'),
    \ }))
