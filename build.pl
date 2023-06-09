#!/bin/perl

use strict;
use Getopt::Long;
use File::Basename;

my $script_dir = dirname(__FILE__);

my $gcc_version = "12.2.0";
my $binutils_version = "2.40";
my $gdb_version = "13.1";
my $cmake_version = "3.26.3";
my $openssl_version = "3.1.0";


my $target;
my $toolchain_only;

GetOptions(
    'target=s' => \$target,
    'toolchain-only' => \$toolchain_only,
    ) or die("Error in command line arguments\n");

if ($target eq "") {
    die("Error: target must be specified\n");
}

my $prefix = "/usr/sysroot-$target";

my $toolchain_image = "kernel-toolchain:$target";
my $image = "kernel:$target";

open(LT, '>', "/tmp/current_toolchain_img") or die $!;
print LT $toolchain_image;
close(LT);

open(LT, '>', "/tmp/current_img") or die $!;
print LT $image;
close(LT);

print "Building kernel toolchain for target $target as image $toolchain_image.\n";
print "USE gcc $gcc_version.\n";
print "USE binutils $binutils_version.\n";
print "USE cmake $cmake_version.\n";

print "PREFIX $prefix.\n";

my $build_env = "MSYS_NO_PATHCONV=1";

if (system("$build_env docker build \\
    -f $script_dir/Dockerfile.toolchain \\
    --build-arg PREFIX=$prefix \\
    --build-arg TARGET=$target \\
    --build-arg GCC_VERSION=$gcc_version \\
    --build-arg GDB_VERSION=$gdb_version \\
    --build-arg BINUTILS_VERSION=$binutils_version \\
    --build-arg CMAKE_VERSION=$cmake_version \\
    --build-arg OPENSSL_VERSION=$openssl_version \\
    -t $toolchain_image \\
    $script_dir") != 0) {
    die("Error: Kernel toolchain build failed due to previous error\n");
}

unless ($toolchain_only) {
    print "Building kernel for target $target as image $image.\n";

    if (system("$build_env docker build \\
        --build-arg TARGET=$target \\
        -t $image \\
        $script_dir") != 0) {
        die("Error: Kernel build failed due to previous error\n");
    }
} else {
    print "Skiping kernel build because of --toolchain-only flag specified\n";
}
