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
vim.o.fileformats = vim.fn.has("win64") and "dos,unix,mac" or "unix,dos,mac"

-- シェルを指定する（Windows環境用）
if vim.fn.has("win64") then
	vim.o.shell = string.gsub(vim.fn.system("powershell -c (gcm nu).Definition"), "\n", "")
	vim.o.shellcmdflag = "-c nu --login"
end

-- マウスに反応しないようにする
-- set mouse=
vim.o.mouse = ""

-- 左右のカーソル移動で行をまたがないようにする
-- set whichwrap=
vim.o.whichwrap = ""

-- インサートモードでのbackspaceで改行とautoindentを削除できないようにする
-- set backspace=
vim.o.backspace = "start"


-- 文字幅の問題はターミナルエミュレータのHyperに丸投げする
-- vim.o.ambiwidth = "double"
-- vim.fn.setcellwidths {
--   {0x100, 0xFFFFFF, 2,},
-- }
