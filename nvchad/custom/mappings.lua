---@type MappingsTable
local M = {}

M.general = {
  n = {
    [";"] = { ":", "enter command mode", opts = { nowait = true } },
  },
  i = {
    [";<ESC>"] = { "<Cmd>lua require('insert-end-semicolon').insertEndSemicolon()<CR><ESC>", "insert 「;」 at the end of line", opts = { nowait = true } },
  },
}

-- more keybinds!

return M
