docker rm -f svc-texture || true
docker run --detach --publish 63998:63998 --name svc-texture gwihlidal/svc-texture:1