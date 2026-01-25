set number

"color elflord
color desert

set colorcolumn=80

" Tab: Configuration
set softtabstop=4   " Number of spaces a Tab counts for when editing
set smarttab        " Makes Tab key insert spaces or tabs according to context

" Tab: Expection (Makefile)
autocmd FileType make setlocal noexpandtab

call plug#begin()

" List your plugins here
Plug 'tpope/vim-sensible'
Plug 'rust-lang/rust.vim'
Plug 'cespare/vim-toml', { 'branch': 'main' }
Plug 'kaarmu/typst.vim'

call plug#end()

highlight ColorColumn ctermbg=250 guibg=grey
