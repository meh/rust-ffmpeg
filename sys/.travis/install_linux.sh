#!/bin/bash

sudo apt-get update -q
# From https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu
sudo apt-get -y --force-yes install autoconf automake build-essential libass-dev libfreetype6-dev libsdl1.2-dev libtheora-dev libtool libva-dev libvdpau-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev pkg-config texinfo zlib1g-dev
sudo apt-get install yasm
pushd ~
git clone https://github.com/FFmpeg/FFmpeg.git
cd FFmpeg
git checkout release/3.2
mkdir ~/FFmpeg-build
cd ~/FFmpeg-build
../FFmpeg/configure --disable-ffprobe --disable-ffserver --disable-doc --enable-avresample
make -j
sudo make install
make distclean
popd
