#!/bin/bash

rm -rf scripts/output/client* scripts/server

mkdir -p scripts/output/client-a scripts/output/client-b scripts/output/server

ssh-keygen -t ed25519 -C "gitstake-userA@paastech.fr" -f scripts/output/client-a/id_ed25519 -q -N ""

ssh-keygen -t ed25519 -C "gitstake-userB@paastech.fr" -f scripts/output/client-b/id_ed25519 -q -N ""

echo "command=\"/app/gitstake userA\" $(cat scripts/output/client-a/id_ed25519.pub)" > scripts/output/server/authorized_keys

echo "command=\"/app/gitstake userB\" $(cat scripts/output/client-b/id_ed25519.pub)" >> scripts/output/server/authorized_keys
