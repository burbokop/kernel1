ARG TARGET
FROM kernel-toolchain:$TARGET as kernel-build

COPY . /opt/kernel

RUN cd /opt && \
    mkdir -p kernel-build && \
    cd kernel-build && \
    cmake \
        -DCMAKE_BUILD_TYPE=Release \
        ../kernel

RUN cd /opt/kernel-build && make iso -j $(nproc)
RUN cd /opt/kernel-build && cpack -G DEB

FROM debian:bookworm as kernel

RUN apt update && apt install -y \
    qemu-system-x86 \
    dbus-x11 \
    genisoimage

COPY ./tools/choose-qemu /opt/qemu
COPY --from=kernel-build /packages/binutils*.deb /tmp
COPY --from=kernel-build /opt/kernel-build/*.deb /tmp
RUN dpkg -i /tmp/*.deb
RUN  dbus-uuidgen > /etc/machine-id

ARG TARGET

RUN echo `/opt/qemu ${TARGET}` -cdrom /bin/kernel1.iso -d cpu_reset -monitor stdio > /run-qemu
RUN chmod +x /run-qemu

CMD isoinfo -l -i /bin/kernel1.iso && /run-qemu
