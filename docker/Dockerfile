FROM ubuntu:18.04
RUN apt update
RUN apt-get -y install gpgv gnupg2 ca-certificates
RUN echo "deb [arch=amd64] https://repo.rldsoftware.nl bionic main" > /etc/apt/sources.list.d/localrepo.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys B6E127E5BCAB9E8D
RUN apt-get update
RUN apt-get install -y contactbook-api

EXPOSE 3030

