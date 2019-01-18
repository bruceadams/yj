FROM scratch

COPY target/yj.linux /yj

ENTRYPOINT [ "/yj" ]
