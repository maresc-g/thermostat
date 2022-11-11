# https://solarianprogrammer.com/2018/05/06/building-gcc-cross-compiler-raspberry-pi/

# Ubuntu 18.04 at the time of writing (2019-04-02)
FROM ubuntu:latest

# Install some tools and compilers + clean up
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install dialog apt-utils -y
    
RUN apt-get install -y git wget gcc-8 g++-8 cmake gdb gdbserver bzip2 curl rsync && \
    apt-get -y clean autoclean && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Use GCC 8 as the default
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-8 999 \
 && update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-8 999 \
 && update-alternatives --install /usr/bin/cc  cc  /usr/bin/gcc-8 999 \
 && update-alternatives --install /usr/bin/c++ c++ /usr/bin/g++-8 999

# Add a user called `develop`
RUN useradd -ms /bin/bash develop
RUN echo "develop   ALL=(ALL:ALL) ALL" >> /etc/sudoers

WORKDIR /home/develop

# Download and extract GCC
RUN wget https://ftp.gnu.org/gnu/gcc/gcc-8.3.0/gcc-8.3.0.tar.xz && \
    tar xf gcc-8.3.0.tar.xz && \
    rm gcc-8.3.0.tar.xz
# Download and extract LibC
RUN wget https://ftp.gnu.org/gnu/libc/glibc-2.29.tar.bz2 && \
    tar xjf glibc-2.29.tar.bz2 && \
    rm glibc-2.29.tar.bz2
# Download and extract BinUtils
RUN wget https://ftp.gnu.org/gnu/binutils/binutils-2.32.tar.bz2 && \
    tar xjf binutils-2.32.tar.bz2 && \
    rm binutils-2.32.tar.bz2
# Download the GCC prerequisites
RUN cd gcc-8.3.0 && contrib/download_prerequisites && rm *.tar.*

# Build BinUtils
RUN mkdir -p /opt/cross-pi-gcc
WORKDIR /home/develop/build-binutils
RUN ../binutils-2.32/configure \
        --prefix=/opt/cross-pi-gcc --target=arm-linux-gnueabi \
        --with-arch=armv6 --with-fpu=vfp \
        --disable-multilib
RUN make -j$(nproc)
RUN make install

# Build the first part of GCC
WORKDIR /home/develop/build-gcc
RUN ../gcc-8.3.0/configure \
        --prefix=/opt/cross-pi-gcc \
        --target=arm-linux-gnueabi \
        --enable-languages=c,c++,fortran \
        --with-arch=armv6 --with-fpu=vfp \
        --disable-multilib
RUN make -j$(nproc) all-gcc
RUN make install-gcc
ENV PATH=/opt/cross-pi-gcc/bin:${PATH}

# Install dependencies
RUN apt-get update && \
    apt-get install -y gawk bison python3 && \
    apt-get clean autoclean && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Download and install the Linux headers
WORKDIR /home/develop
RUN git clone --depth=1 https://github.com/raspberrypi/linux
WORKDIR /home/develop/linux
ENV KERNEL=kernel7
RUN make ARCH=arm INSTALL_HDR_PATH=/opt/cross-pi-gcc/arm-linux-gnueabi headers_install

# Build GLIBC
WORKDIR /home/develop/build-glibc
RUN ../glibc-2.29/configure \
        --prefix=/opt/cross-pi-gcc/arm-linux-gnueabi \
        --build=$MACHTYPE --host=arm-linux-gnueabi --target=arm-linux-gnueabi \
        --with-arch=armv6 --with-fpu=vfp \
        --with-headers=/opt/cross-pi-gcc/arm-linux-gnueabi/include \
        --disable-multilib libc_cv_forced_unwind=yes
RUN make install-bootstrap-headers=yes install-headers
RUN make -j8 csu/subdir_lib
RUN install csu/crt1.o csu/crti.o csu/crtn.o /opt/cross-pi-gcc/arm-linux-gnueabi/lib
RUN arm-linux-gnueabi-gcc -nostdlib -nostartfiles -shared -x c /dev/null \
        -o /opt/cross-pi-gcc/arm-linux-gnueabi/lib/libc.so
RUN touch /opt/cross-pi-gcc/arm-linux-gnueabi/include/gnu/stubs.h

# Continue building GCC
WORKDIR /home/develop/build-gcc
RUN make -j$(nproc) all-target-libgcc
RUN make install-target-libgcc

# Finish building GLIBC
WORKDIR /home/develop/build-glibc
RUN make -j$(nproc)
RUN make install

# Finish building GCC
WORKDIR /home/develop/build-gcc
RUN make -j$(nproc)
RUN make install

USER develop

WORKDIR /home/develop

RUN curl https://sh.rustup.rs -sSf > rust.sh && \
    sh rust.sh -y
    
RUN .cargo/bin/rustup target add arm-unknown-linux-gnueabi

WORKDIR /home/develop/server

CMD ["/home/develop/.cargo/bin/cargo", "build", "--release", "--target", "arm-unknown-linux-gnueabi"]
