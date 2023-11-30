#!/bin/bash
cd cd ..\..\docker_server\
docker-compose up -d
#pg_isready comes with an installation of PostgreSQL
# else: until docker run -it postgres --add-host host.docker.internal:hostgateway docker.io/postgres:14-alpine -h localhost -U username 
pg_isready
until pg_isready -h localhost -p 5432 -U username
do
  echo "Waiting for postgres to spin up..."
  sleep 4;
done
echo "docker is now running"
docker-compose down