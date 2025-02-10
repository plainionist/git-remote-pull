# git-remote-pull

Manual trigger pull on remote repository

# Deployment

```
sudo docker build -t git-remote-pull .
```

```
sudo docker stop git-remote-pull
sudo docker rm git-remote-pull
```

```
docker run -d --restart=unless-stopped --name git-remote-pull -p 9999:9999 -v /path/to/your/local/repo:/repo git-remote-pull
```
