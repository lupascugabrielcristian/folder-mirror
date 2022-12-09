FROM debian:buster

COPY ./target/debug/foldermirror /usr/bin
COPY output.xml /tmp/
#ENTRYPOINT ["./foldermirror import output.xml"]
CMD /bin/bash
