# Twitter Clone API 

## Install postgres

```bash
sudo apt install postgresql postgresql-contrib libpq-dev
sudo -u postgres createuser -P --interactive
```

## Diesel setup

```bash
echo DATABASE_URL=postgres://username:password@localhost:5432/twitter_clone > .env

cargo install diesel_cli --no-default-features --features postgres

diesel setup

diesel migration generate create_<modelsName>


```

```bash

curl -X POST -d "my first tweet" -H "Content-type: application/json" http://localhost:8000/tweets | jq

curl -X GET -H "Content-type: application/json" http://localhost:8000/tweets | jq
```
