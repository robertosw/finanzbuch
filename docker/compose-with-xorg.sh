#!/usr/bin/env bash
echo "Allow docker to attach to host X Server"
xhost +Local:docker
echo 

echo "Starting docker container. Press CTRL+C to stop this container"

docker compose up
# docker exec -it finanzbuch /bin/bash

# echo "Stopping Container"
# docker stop finanzbuch

echo "Removing Allow-Rule from host X Server"
xhost -Local:docker
