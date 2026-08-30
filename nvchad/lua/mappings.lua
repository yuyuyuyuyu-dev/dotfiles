require "nvchad.mappings"

local map = vim.keymap.set

map("n", ";", ":", { desc = "CMD enter command mode" })

for _, key in ipairs { "<C-S-j>", "<C-S-;>" } do
  map({ "i", "n" }, key, "<Nop>", {
    desc = "入力切り替えのショートカットに反応しないようにする",
    nowait = true,
  })
end
