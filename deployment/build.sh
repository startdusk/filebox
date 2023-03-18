DOMAIN=$1
cd ../$DOMAIN
docker build -t filebox/$DOMAIN -f ../deployment/$DOMAIN/Dockerfile .
