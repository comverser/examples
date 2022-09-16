- How do I view all available HDD's/partitions?
```bash
sudo lsblk -o NAME,FSTYPE,SIZE,MOUNTPOINT,LABEL
```

- How do I expand the size of ubuntu--vg-ubuntu--lv?
```bash
sudo vgs
sudo lvs
```
```bash
sudo lvextend --extents +100%FREE /dev/mapper/ubuntu--vg-ubuntu--lv
```
```bash
sudo resize2fs /dev/mapper/ubuntu--vg-ubuntu--lv
```
