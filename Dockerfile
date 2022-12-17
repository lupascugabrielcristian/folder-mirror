# Vreau sa folosesc asta doar sa rulez executabilul pentru a genera structura de foldere
# Copii in interior exacutabilul generat cu build.Dockerfile pentru a se potrivi versiunile de glibc
# ldd --version
FROM debian:buster

RUN mkdir foldermirror
COPY ./target/release/foldermirror /foldermirror
COPY output.xml /foldermirror
#COPY broot /usr/bin


WORKDIR /foldermirror
RUN ./foldermirror import output.xml 2> /tmp/foldermirror.log
CMD /bin/bash

