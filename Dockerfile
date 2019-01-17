FROM scratch

ADD https://github.com/bruceadams/yj/releases/download/0.7.10/yj.linux /yj

ENTRYPOINT [ "/yj" ]
