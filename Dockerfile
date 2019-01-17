FROM scratch

ADD https://github.com/bruceadams/yj/releases/download/$SOURCE_BRANCH/yj.linux /yj

ENTRYPOINT [ "/yj" ]
