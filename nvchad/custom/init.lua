-- local autocmd = vim.api.nvim_create_autocmd

-- Auto resize panes when resizing nvim window
-- autocmd("VimResized", {
--   pattern = "*",
--   command = "tabdo wincmd =",
-- })

-- 参考資料
-- https://neovim.io/doc/user/lua-guide.html#lua-guide

-- 文字コードの設定
-- set fileencodings=
vim.o.fileencodings = "utf-8,cp932,sjis,ec-jp"

-- 改行コードの設定
-- set fileformats=
vim.o.fileformats = vim.fn.has("win64") ~= 0 and "dos,unix,mac" or "unix,dos,mac"

-- マウスに反応しないようにする
-- set mouse=
vim.o.mouse = ""

-- 左右のカーソル移動で行をまたがないようにする
-- set whichwrap=
vim.o.whichwrap = ""

-- インサートモードでのbackspaceで改行とautoindentを削除できないようにする
-- set backspace=
vim.o.backspace = "start"

-- filetypeがshのときはインデントをハードタブにする
vim.api.nvim_create_augroup("sh indent", { clear = true })
vim.api.nvim_create_autocmd({ "FileType" }, {
  pattern = "sh",
  group = "sh indent",
  command = "setlocal noexpandtab"
})
