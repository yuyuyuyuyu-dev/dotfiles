require("nvchad.configs.lspconfig").defaults()

local lspconfig = require "lspconfig"

local servers = {
  "clangd",
  "bashls",
  "pylsp",
  "gopls",
  "rust_analyzer",
  "html",
  "cssls",
  "ts_ls",
}
local nvlsp = require "nvchad.configs.lspconfig"

for _, lsp in ipairs(servers) do
  lspconfig[lsp].setup {
    on_attach = nvlsp.on_attach,
    on_init = nvlsp.on_init,
    capabilities = nvlsp.capabilities,
  }
end

local node_modules = vim.fn.stdpath "data" .. "/mason/packages/angular-language-server/node_modules"
local ngls_cmd = {
  node_modules .. "/@angular/language-server/bin/ngserver",
  "--stdio",
  "--tsProbeLocations",
  node_modules .. "/typescript/lib",
  "--ngProbeLocations",
  node_modules .. "/@angular/language-server/bin",
}

lspconfig.angularls.setup {
  cmd = ngls_cmd,
  on_attach = nvlsp.on_attach,
  capabilities = nvlsp.capabilities,
  on_new_config = function(new_config, _)
    new_config.cmd = ngls_cmd
  end,
}
