# NVM
```bash
# place this after nvm initialization!
autoload -U add-zsh-hook
load-nvmrc() {
  [[ -a .nvmrc ]] || return
  local node_version="$(nvm version)"
  local nvmrc_path="$(nvm_find_nvmrc)"

  if [ -n "$nvmrc_path" ]; then
    local nvmrc_node_version=$(nvm version "$(cat "${nvmrc_path}")")

    if [ "$nvmrc_node_version" = "N/A" ]; then
      nvm install
    elif [ "$nvmrc_node_version" != "$node_version" ]; then
      nvm use
    fi
  elif [ "$node_version" != "$(nvm version default)" ]; then
    echo "Reverting to nvm default version"
    nvm use default
  fi
}
add-zsh-hook chpwd load-nvmrc
load-nvmrc
```

# install
- graphic card driver
- browsers: chrome, brave
- vscode
- hwp
- slack
- discord
- nvm
- anaconda
- printer driver (optional)

# unpacking archive with Korean file names

```bash
unzip -O cp949 koreanFileNames.zip
```

# How do I add an executable to my search path?

1. Create a folder called bin in your home directory.

```bash
mkdir ~/bin
```

2. Add ~/bin to your PATH for all sessions of Bash (the default shell used inside of the terminal).

```bash
vim ~/.zshrc
```

```
export PATH="/home/$USER/bin:$PATH"
```

3. Add either the executable files themselves OR symlinks to the executable into ~/bin

```bash
source ~/.zshrc
```

# share
