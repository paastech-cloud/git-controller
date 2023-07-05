# Ssh keys fuse

rust doc:
https://docs.rs/fuser/latest/fuser/

repo fuser:
https://github.com/cberner/fuser

examples :
https://github.com/cberner/fuser/tree/master/examples

```
cd ssh-keys-fuse
mkdir .ssh
cargo run
ls .ssh
cat .ssh/authorized_keys
```

## how to get db schema

ce script va appliquer les migrations écrites par la team front

```
git clone git@github.com:paastech-cloud/api.git

cd api

yarn

# monter une base de données
docker compose up -d

# modifier le .env du dossier api pour mettre le bon string de connection
DATABASE_URL="postgresql://paastech:paastech@localhost:5432/paastech"

# applique les migrations
npx prisma migrate dev
```
