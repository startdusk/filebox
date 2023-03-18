DOMAIN=$1
cd ../server
docker build -t filebox/$DOMAIN -f ../deployment/$DOMAIN/Dockerfile .
