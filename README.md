# git-remote-pull

Manual trigger pull on remote repository

# Deployment

```
docker build -t git-pull-web .
```

```
docker run -d --restart unless-stopped -p 9999:9999 -v /path/to/your/local/repo:/repo git-pull-web /repo
```