# script to run code coverage check and open in browser
# requires docker, only works on windows (but could easily be extended)
# run something like this on first run:
    # docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "git clone https://github.com/GarrettFechter/rusty-gb.git /volume && cargo tarpaulin --out Lcov"

BRANCH=`git rev-parse --abbrev-ref HEAD`
# docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "cd /volume && git clone https://github.com/GarrettFechter/rusty-gb.git && cd rusty-gb && git pull && git checkout $BRANCH && cargo tarpaulin --out Html"
docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin sh -c "cd /volume && cd rusty-gb && git pull && git checkout $BRANCH && cargo tarpaulin --out Html"

# kinda sketchy to get the container name like this
# but will a new docker container really get created at this very second?
# probably not...
CONTAINER_NAME=`docker ps --last 1 -q --format "{{.Names}}"`
docker cp $CONTAINER_NAME:/volume/rusty-gb/tarpaulin-report.html .

# expects windows environment
explorer.exe tarpaulin-report.html
docker rm $CONTAINER_NAME
