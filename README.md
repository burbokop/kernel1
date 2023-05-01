# KERNEL1
Test x86 kernel

## How to run
```
./run --target i686-elf
```

## Debugging
```
qemu-system-i386 -s -S -no-shutdown -action panic=pause -kernel ./kernel1
/usr/sysroot-i686-elf/bin/i686-elf-gdb --eval-command="target remote localhost:1234" ./kernel1
```
