FROM wq/ubuntu:v20.0

MAINTAINER jackie <dumingyan@cph.citic>

ENV ORG=org1 
ENV CANAME=ca1
ENV ROOT=/data/myrust/phoenix_chain
ENV CAURL=0.0.0.0:9009
ENV INNER=0.0.0.0:8000
ENV RPC=0.0.0.0:9000
ENV FRIENDS=0.0.0.0:8001
ENV CRTDIR=$ROOT/data/crt
ENV TLSCERT=$ROOT/data/tls/cert.pem
ENV TLSKEY=$ROOT/data/tls/private.key
ENV CADIR=$ROOT/ca/data/child/$ORG
ENV CACERT=$CADIR/cert
ENV CAKEY=$CADIR/sk.pem
ENV CAPUB=$CADIR/pk.pem
ENV SLEDDIR=${ROOT}/data/db
ENV CONTRACTDIR=${ROOT}/data/contract
ENV BLOCKDIR=${ROOT}/data/block/
ENV ECNMETHOD=sm
ENV SAVE=txt
ENV INNERTRANS=sm

ADD phoenix_chain /data/myrust/

EXPOSE 8000
EXPOSE 9000
EXPOSE 9009
ENTRYPOINT  ["./data/myrust/phoenix_chain/phoenix_chain"]
