# StaticUnit

A web server for frontend and static files.

## Getting Started

Start with command (*Example*):

```bash
docker run -d \
  --restart=always \
  --name=staticunit \
  -v './dist:/app' \
  -p '8000:80' \
  repigeons/staticunit
```
