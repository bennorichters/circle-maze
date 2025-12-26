# Deployment Guide for Dokku

This guide covers deploying the Circle Maze WebAssembly app to Dokku using Docker.

## Prerequisites

- Dokku installed on your server (v0.27+)
- SSH access to your Dokku server
- Docker plugin enabled on Dokku (enabled by default)

## Initial Setup

### 1. Create the Dokku App

SSH into your Dokku server and create the app:

```bash
dokku apps:create circle-maze
```

### 2. Configure Environment Variables

Set the port (Dokku will override this automatically):

```bash
dokku config:set circle-maze PORT=5000
```

### 3. Set Up Domain (Optional)

```bash
dokku domains:set circle-maze yourdomain.com
```

### 4. Add Git Remote

On your local machine, add the Dokku remote:

```bash
git remote add dokku dokku@your-server.com:circle-maze
```

## Deployment

### First Deployment

Push your code to Dokku:

```bash
git push dokku claude/deploy-wasm-dokku-zHcve:main
```

Dokku will:
1. Detect the Dockerfile
2. Build the Rust/WASM code in the builder stage
3. Create a production image with nginx
4. Deploy the container

### Subsequent Deployments

After making changes:

```bash
git add .
git commit -m "Your commit message"
git push dokku <your-branch>:main
```

## Post-Deployment Configuration

### Enable Zero-Downtime Deployments

```bash
dokku checks:enable circle-maze
```

### Set Up SSL with Let's Encrypt

```bash
dokku letsencrypt:enable circle-maze
dokku letsencrypt:cron-job --add
```

### Configure Proxy Ports

```bash
dokku proxy:ports-set circle-maze http:80:5000 https:443:5000
```

## Testing the Deployment

### Check Deployment Status

```bash
dokku ps:report circle-maze
```

### View Logs

```bash
dokku logs circle-maze
dokku logs circle-maze --tail
```

### Test the Application

```bash
curl -I http://your-domain.com
```

Or visit http://your-domain.com in your browser.

## Local Docker Testing

Before deploying to Dokku, you can test the Docker build locally:

```bash
docker build -t circle-maze:test .

docker run -p 8080:5000 -e PORT=5000 circle-maze:test
```

Visit http://localhost:8080 to test.

## Troubleshooting

### Build Fails

Check the build logs:

```bash
dokku logs circle-maze --tail
```

### App Won't Start

Check if the container is running:

```bash
dokku ps:inspect circle-maze
```

### Rebuild Without Cache

```bash
dokku ps:rebuild circle-maze --no-cache
```

## Continuous Deployment with GitHub Actions

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to Dokku

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Deploy to Dokku
        uses: dokku/github-action@master
        with:
          git_remote_url: 'ssh://dokku@your-server.com:22/circle-maze'
          ssh_private_key: ${{ secrets.DOKKU_SSH_KEY }}
          branch: main
```

Add your SSH private key as a GitHub secret named `DOKKU_SSH_KEY`.

## Architecture

The deployment uses a multi-stage Docker build:

1. **Builder Stage**: Rust container that compiles WASM with wasm-pack
2. **Production Stage**: Lightweight nginx:alpine that serves static files

This approach:
- Minimizes final image size
- Optimizes build caching
- Ensures reproducible builds
- Provides fast, reliable static file serving

## Updating Dependencies

When updating Rust dependencies:

```bash
cargo update
git add Cargo.lock
git commit -m "Update dependencies"
git push dokku <your-branch>:main
```

The Docker build will automatically use the updated dependencies.
