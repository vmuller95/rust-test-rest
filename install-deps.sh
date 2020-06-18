#!/bin/sh

git clone --depth=1 https://github.com/google/skia.git
sudo apt install ninja-build
sudo apt-get install mesa-common-dev
cd skia
python2 tools/git-sync-deps
bin/gn gen out/clang --args='cc="clang" cxx="clang++" is_official_build=true skia_use_system_libpng=false skia_use_system_harfbuzz=false skia_use_system_libjpeg_turbo=false skia_use_system_libwebp=false skia_use_system_expat=false skia_use_system_icu=false skia_use_system_freetype2=false skia_use_fontconfig=false skia_use_system_zlib=false is_debug=false'
ninja -C out/clang
cp -r include/ ../previews-generator/
cp out/clang/libskia.a ../previews-generator/
