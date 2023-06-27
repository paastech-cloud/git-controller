#!/bin/bash

rm -rf scripts/output/client* scripts/server

mkdir -p scripts/output/client-a scripts/output/client-b scripts/output/server

ssh-keygen -t ed25519 -C "userA@user.fr" -f scripts/output/client-a/id_ed25519 -q -N ""

ssh-keygen -t ed25519 -C "userB@user.fr" -f scripts/output/client-b/id_ed25519 -q -N ""

echo "command=\"/usr/bin/git-auth-layer 1\" $(cat scripts/output/client-a/id_ed25519.pub)" > scripts/output/server/authorized_keys

echo "command=\"/usr/bin/git-auth-layer 2\" $(cat scripts/output/client-b/id_ed25519.pub)" >> scripts/output/server/authorized_keys