FROM debian:buster

RUN mkdir foldermirror
COPY ./target/release/foldermirror /foldermirror
COPY output.xml /foldermirror
COPY broot /usr/bin


WORKDIR /foldermirror
RUN ./foldermirror import output.xml 2> /tmp/foldermirror.log
CMD /bin/bash

