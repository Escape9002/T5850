#! /bin/bash

git clone git@github.com:freehuntx/gpn-tron.git

cd gpn-tron

docker compose up -d 
docker compose logs -f 