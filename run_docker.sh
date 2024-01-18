docker stop gembalang
docker rm gembalang
docker build . -t gembalang --network host
docker run --name gembalang -d -i -t gembalang /bin/sh
docker exec -it gembalang bash