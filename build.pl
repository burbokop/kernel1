#!/bin/perl

use strict;
use Getopt::Long;

my $gcc_version = "12.2.0";
my $binutils_version = "2.40";
my $cmake_version = "3.26.3";

my $prefix = "/usr";

my $target;

GetOptions("target=s" => \$target)
or die("Error in command line arguments\n");

if ($target eq "") {
    die("Error: target must be specified\n");
} elsif ($target ne "x86_64-elf" && $target ne "i686-elf") {
    die("Error: only x86_64-elf and i686-elf targets are available\n");
}

my $image = "kernel:$target";

open(LT, '>', "/tmp/current_img") or die $!;
print LT $image;
close(LT);

print "Building kernel for target $target as image $image.\n";
print "USE gcc $gcc_version.\n";
print "USE binutils $binutils_version.\n";
print "USE cmake $cmake_version.\n";

`docker build \\
    --build-arg PREFIX=$prefix \\
    --build-arg TARGET=$target \\
    --build-arg GCC_VERSION=$gcc_version \\
    --build-arg BINUTILS_VERSION=$binutils_version \\
    --build-arg CMAKE_VERSION=$cmake_version \\
    -t $image .`;
