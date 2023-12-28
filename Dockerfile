FROM alpine

WORKDIR /usr/gembalang

RUN apk update && apk upgrade && apk add --update alpine-sdk && \
    apk add --no-cache bash git make cmake 
RUN apk add flex && apk add bison
RUN apk add --no-cache rust cargo

COPY . .
