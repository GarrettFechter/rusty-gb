# script to run code coverage check and open in browser
# requires docker, only works on windows (but could easily be extended)

docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "cargo tarpaulin --out Html"

# kinda sketchy to get the container name like this
# but will a new docker container really get created at this very second?
# probably not...
CONTAINER_NAME=`docker ps --last 1 -q --format "{{.Names}}"`
docker cp $CONTAINER_NAME:/volume/tarpaulin-report.html .

# expects windows environment
explorer.exe tarpaulin-report.html
docker rm $CONTAINER_NAME
