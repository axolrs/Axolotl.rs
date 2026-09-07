-- Owner: PascalElixir / axolrs (GitHub org)
-- File: editors/nvim-axolotl/lua/axolotl/init.lua - the Neovim plugin for Axolotl.

local M = {}

-- Set up the Axolotl plugin with the given options.
function M.setup(opts)
    opts = opts or {}

    -- Configure the Gills LSP server.
    vim.lsp.config("axol_analyzer", {
        cmd = opts.cmd or { "axol-analyzer" },
        filetypes = { "axol" },
        root_markers = { "Bucket.jsonc", "Cargo.toml", ".git" },
        settings = opts.settings or {},
    })
    vim.lsp.enable("axol_analyzer")

    -- Register the tree-sitter parser if nvim-treesitter is available.
    local ok, parsers = pcall(require, "nvim-treesitter.parsers")
    if ok then
        local parser_configs = parsers.get_parser_configs()
        parser_configs.axol = {
            install_info = {
                url = opts.grammar_path or (vim.fn.getcwd() .. "/tooling/tree-sitter-axol"),
                files = { "src/parser.c" },
                branch = "main",
            },
            filetype = "axol",
        }
    end

    -- Define the :Axolotl command namespace.
    vim.api.nvim_create_user_command("Axolotl", function(args)
        local sub = args.fargs[1] or "help"
        if sub == "run" then
            vim.fn.termopen("bucket run")
        elseif sub == "build" then
            vim.fn.termopen("bucket build --release")
        elseif sub == "test" then
            vim.fn.termopen("bucket test")
        elseif sub == "fmt" then
            vim.fn.termopen("bucket fmt")
        elseif sub == "lint" then
            vim.fn.termopen("bucket lint")
        elseif sub == "fix" then
            vim.fn.termopen("bucket fix")
        elseif sub == "repl" then
            vim.fn.termopen("axol-hot-runner repl")
        elseif sub == "gills" then
            local clients = vim.lsp.get_clients({ name = "axol_analyzer" })
            if #clients == 0 then
                print("Gills LSP: not running")
            else
                print("Gills LSP: running (" .. #clients .. " client(s))")
            end
        else
            print("Axolotl commands: run, build, test, fmt, lint, fix, repl, gills")
        end
    end, { nargs = "*", desc = "Axolotl commands" })

    -- Default settings.
    if opts.inlay_hints == nil then opts.inlay_hints = { enabled = true } end
    if opts.tree_sitter_highlight == nil then opts.tree_sitter_highlight = { enabled = true } end
    if opts.format_on_save == nil then opts.format_on_save = { enabled = true, tool = "Salamander" } end
    if opts.on_save == nil then opts.on_save = { run = false } end
end

return M
