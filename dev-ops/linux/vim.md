# the ultimate vim configuration

```bash
git clone --depth=1 https://github.com/amix/vimrc.git ~/.vim_runtime
sh ~/.vim_runtime/install_awesome_vimrc.sh
```

## Add your own customizations in ~/.vim_runtime/my_configs.vim

```bash
autocmd filetype javascript nnoremap <F5> :w <bar> exec '!node '.shellescape('%')<CR>
```
