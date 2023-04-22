ARG TARGET
FROM kernel-toolchain:$TARGET as kernel-build

COPY . /opt/kernel

RUN cd /opt && \
    mkdir -p kernel-build && \
    cd kernel-build && \
    cmake ../kernel

RUN cd /opt/kernel-build && make iso -j $(nproc)
RUN cd /opt/kernel-build && cpack -G DEB

FROM debian:bookworm as kernel

RUN apt update && apt install -y \
    qemu-system-x86 \
    dbus-x11

COPY ./tools/choose-qemu /opt/qemu
COPY --from=kernel-build /tmp/binutils*.deb /tmp
COPY --from=kernel-build /opt/kernel-build/*.deb /tmp
RUN dpkg -i /tmp/*.deb
RUN  dbus-uuidgen > /etc/machine-id

ARG TARGET

RUN echo `/opt/qemu ${TARGET}` -kernel /bin/kernel1.bin > /run-qemu
RUN chmod +x /run-qemu

CMD /run-qemu
