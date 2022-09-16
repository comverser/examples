# Disable Suspend and Hibernation in Linux

```bash
sudo systemctl mask sleep.target suspend.target hibernate.target hybrid-sleep.target
sudo systemctl status sleep.target suspend.target hibernate.target hybrid-sleep.target
```

# How to disable the "unlock your keyring" popup?

```bash
rm ~/.local/share/keyrings/login.keyring
```

# Fix low resolution of unknown display
- Find display name
```bash
xrandr
```
- Set resolution (if the name is HDMI-1)
```bash
xrandr --addmode HDMI-1 1920x1080
```


# A2000U wireless setting

```bash
git clone https://github.com/aircrack-ng/rtl8812au.git
```

```bash
cd rtl8812au
git checkout v5.7.0
sudo make && sudo make install
```

```bash
sudo reboot
```
