FROM rust

WORKDIR /usr/gembalang

RUN apt-get update 
RUN apt-get install -y flex && apt-get install -y  bison
RUN apt-get install -y libcln-dev

COPY . .
