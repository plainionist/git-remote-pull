# git-remote-pull

Manual trigger pull on remote repository

# Deployment

```
docker build -t git-remote-pull .
```

```
docker run -d --restart=unless-stopped --name git-remote-pull -p 9999:9999 -v /path/to/your/local/repo:/repo git-remote-pull
```
