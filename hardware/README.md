# Hardware

## Monitor GPU utilization

```bash
watch -d -n 1 nvidia-smi
```

## Set Environment Variables

To edit your `.bashrc` file:

```bash
sudo hx ~/.bashrc
```

Add the following line to set the `LD_LIBRARY_PATH`:

```text
export PATH=$PATH:/usr/local/cuda-12.2/bin
export CUDADIR=/usr/local/cuda-12.2
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/cuda-12.2/lib64/
```

Apply the changes by sourcing the `.bashrc` file:

```bash
source ~/.bashrc
```

## Remove CUDA on WSL2

To uninstall CUDA, run the following commands:

```bash
sudo rm -rf /usr/local/cuda*
sudo apt-get --purge -y remove 'cuda*'
sudo apt-get autoremove --purge -y 'cuda*'
sudo apt-get autoclean
```

Verify the removal of CUDA packages:

```bash
sudo dpkg -l | grep cuda
```

### Remove NVIDIA Drivers

To uninstall NVIDIA drivers, use these commands:

```bash
sudo apt-get --purge -y remove 'nvidia*'
sudo apt-get autoremove --purge -y 'nvidia*'
sudo apt-get autoclean
```

Verify the removal of NVIDIA packages:

```bash
sudo dpkg -l | grep nvidia
```
