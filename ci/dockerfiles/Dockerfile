FROM node:12.16.2

MAINTAINER Fariz Rizaldy <fariz@evilfactory.id>

WORKDIR /koalaci

COPY entrypoint.sh /

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]