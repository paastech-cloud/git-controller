#!/bin/bash

# add all keys to ssh-agent starting with id_ed25519 and not ending with .pub
eval "$(ssh-agent -s)" && ssh-add "$(ls -1 /root/.ssh/id_ed25519* | grep -v .pub)"

tail "-f" "/dev/null"
