FROM jodogne/orthanc-plugins:1.5.8

WORKDIR /orthanc-gcp-storage

RUN apt-get update && apt-get install -y build-essential git cmake autoconf libtool libssl-dev libcurl4-openssl-dev pkg-config mingw-w64

COPY . .

CMD []