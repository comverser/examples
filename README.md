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
