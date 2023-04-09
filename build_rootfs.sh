#!/bin/env sh
set -x

RUSTFLAGS="-C target-feature=+crt-static" cargo build --release 2> /dev/null

rootfs=rootfs
rb_loc=${rootfs}/usr/bin/rustybox

# Build rootfs directory structure
mkdir -p ${rootfs}/usr/bin

# Create symlinks in rootfs
cp target/release/rustybox ${rb_loc}
utilities=$(./${rootfs}/usr/bin/rustybox)
for util in ${utilities}
do
    ln -sf rustybox ${rootfs}/usr/bin/${util}
done

# ln -s usr/bin ${rootfs}/bin
# ln -s usr/bin ${rootfs}/sbin
# ln -s usr/bin ${rootfs}/usr/sbin

