### without root permissions
```bash
sudo groupadd docker
sudo usermod -aG docker $USER
```
Then reboot

### frequently used commands
```bash
sudo docker image ls
```

### docker $ jupyter
example (image name is lippoldt331/maskr-cnn)
```bash
sudo docker run -it --gpus all -p 8888:8888 -p 6006:6006 -v ~/:/host cudachen/mask-rcnn-docker jupyter notebook --ip 0.0.0.0 --port 8888 --allow-root /host
```
```bash
sudo docker run -it --gpus all -p 8888:8888 -p 6006:6006 -v ~/:/host lippoldt331/maskr-cnn jupyter lab --ip 0.0.0.0 --port 8888 --allow-root /host
```

### How to copy Docker images from one host to another without using a repository
You will need to save the Docker image as a tar file:
```bash
sudo docker save -o <path for generated tar file> <image name>
```
Then copy your image to a new system with regular file transfer tools such as cp, scp or rsync(preferred for big files). After that you will have to load the image into Docker:
```bash
sudo docker load -i <path to image tar file>
```
You should add filename (not just directory) with -o, for example:
```bash
sudo docker save -o c:/myfile.tar centos:16
```


### Run Docker without sudo Linux
```bash
sudo groupadd docker
sudo usermod -aG docker $USER
```
- Log out and log back to activate the group membership in Linux or you can run


### Manage containers
- List all containers
``` bash
sudo docker ps -a
```

### etc


``` bash
sudo docker volume ls
```

### Clean up
- Why the "none" image appears in Docker and how can we avoid it
```bash
docker rmi $(docker images -f "dangling=true" -q)
```

- Kill all running containers

``` bash
sudo docker kill $(sudo docker ps -q)
``` 

- Prune your entire system

``` bash
sudo docker system prune -a --volumes
```
