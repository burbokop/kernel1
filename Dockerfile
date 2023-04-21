FROM gcc:12.2.0-bullseye AS cc-base

RUN apt update && apt install -y \
    bison \
    flex \
    texinfo \
    checkinstall \
    libgmp3-dev \
    libmpc-dev \
    libmpfr-dev \
    libisl-dev
  
FROM rust:1.69-bullseye as tools-build

COPY ./tools /opt
RUN cd /opt/replaceenv && cargo build --release

FROM cc-base as binutils-build

ARG BINUTILS_VERSION

RUN wget https://ftp.gnu.org/gnu/binutils/binutils-${BINUTILS_VERSION}.tar.xz -O /tmp/binutils.tar.gz
RUN tar -xvf /tmp/binutils.tar.gz -C /opt

ARG PREFIX
ARG TARGET

RUN cd /opt && \
    mkdir -p binutils-build && \
    cd binutils-build && \
    ../binutils-${BINUTILS_VERSION}/configure \
        --prefix=$PREFIX \
        --target=$TARGET \
        --enable-interwork \
        --enable-multilib \
        --disable-nls \
        --disable-werror

RUN cd /opt/binutils-build && make all -j $(nproc)
RUN cd /opt/binutils-build && checkinstall --install=no --fstrans=no --pkgname=binutils --pkgversion=${BINUTILS_VERSION} -y

FROM cc-base as gcc-build

ARG GCC_VERSION

RUN wget ftp://ftp.lip6.fr/pub/gcc/releases/gcc-${GCC_VERSION}/gcc-${GCC_VERSION}.tar.gz -O /tmp/gcc.tar.gz
RUN tar -xvf /tmp/gcc.tar.gz -C /opt
RUN cd /opt/gcc-${GCC_VERSION} && ./contrib/download_prerequisites

#COPY --from=binutils-build /opt/binutils-build/*.deb /tmp
#RUN dpkg -i /tmp/*.deb

COPY --from=binutils-build /opt /tmp
RUN cd /tmp/binutils-build && make install

ARG PREFIX
ARG TARGET

RUN cd /opt && \
    mkdir -p gcc-build && \
    cd gcc-build && \
    ../gcc-${GCC_VERSION}/configure \
        --prefix=$PREFIX \
        --target=$TARGET \
        --disable-nls \
        --enable-languages=c,c++ \
        --without-headers \
        --enable-interwork \
        --enable-multilib \
        --with-gmp=/usr \
        --with-mpc=/opt/local \
        --with-mpfr=/opt/local

RUN cd /opt/gcc-build && make all-gcc -j $(nproc)
RUN cd /opt/gcc-build && make all-target-libgcc -j $(nproc)
#RUN cd /opt/gcc-build && make all-target-libstdc++-v3 -j $(nproc)

COPY --from=tools-build /opt/replaceenv/target/release/replaceenv /usr/bin
COPY ./toolchain.template /opt
RUN replaceenv /opt/toolchain.template > /opt/gcc-build/toolchain.cmake

RUN cd /opt/gcc-build && checkinstall \
    --install=no \
    --fstrans=no \
    --pkgname=gcc \
    --pkgversion=${GCC_VERSION} \
    -y make \
        install-gcc \
        install-target-libgcc 
        #\
        #install-target-libstdc++-v3

#RUN cd /opt/gcc-build && make all -j $(nproc)
#RUN cd /opt/gcc-build && checkinstall --install=no --fstrans=no --pkgname=gcc --pkgversion=${GCC_VERSION} -y make all

FROM cc-base as cmake-build

ARG CMAKE_VERSION

RUN wget https://github.com/Kitware/CMake/archive/refs/tags/v${CMAKE_VERSION}.tar.gz -O /tmp/cmake.tar.gz
RUN tar -xvf /tmp/cmake.tar.gz -C /opt

RUN cd /opt && \
    mkdir -p cmake-build && \
    cd cmake-build && \
    ../CMake-${CMAKE_VERSION}/bootstrap

RUN cd /opt/cmake-build && make -j $(nproc)
RUN cd /opt/cmake-build && ./bin/cpack -G DEB
RUN cd /opt/cmake-build && apt download libssl1.1

FROM debian:bookworm as kernel-build

RUN apt update && apt install -y \
    grub \
    grub-pc-bin \
    xorriso \
    libmpc3 \
    libfl2 \
    make

COPY --from=binutils-build /opt/binutils-build/*.deb /tmp
COPY --from=gcc-build /opt/gcc-build/*.deb /tmp
ARG PREFIX
COPY --from=gcc-build /opt/gcc-build/toolchain.cmake ${PREFIX}/lib
COPY --from=cmake-build /opt/cmake-build/*.deb /tmp

RUN dpkg -i /tmp/*.deb

ARG TARGET

COPY . /opt/kernel

ENV ASM=${PREFIX}/bin/${TARGET}-as
ENV CC=${PREFIX}/bin/${TARGET}-gcc
ENV CXX=${PREFIX}/bin/${TARGET}-g++
ENV CMAKE_TOOLCHAIN_FILE=${PREFIX}/lib/toolchain.cmake

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
COPY --from=binutils-build /opt/binutils-build/*.deb /tmp
COPY --from=kernel-build /opt/kernel-build/*.deb /tmp
RUN dpkg -i /tmp/*.deb
RUN  dbus-uuidgen > /etc/machine-id

ARG TARGET

RUN echo `/opt/qemu ${TARGET}` -kernel /bin/kernel1.bin > /run-qemu
RUN chmod +x /run-qemu

CMD /run-qemu
