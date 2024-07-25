#!/bin/sh

flatpak-spawn --host --watch-bus flatpak build --with-appdir --allow=devel --bind-mount=/run/user/1000/doc=/run/user/1000/doc/by-app/org.gnome.Example --share=network --share=ipc --socket=fallback-x11 --device=dri --socket=wayland --talk-name=org.freedesktop.portal.* --talk-name=org.a11y.Bus --bind-mount=/run/flatpak/at-spi-bus=/run/user/1000/at-spi/bus --env=AT_SPI_BUS_ADDRESS=unix:path=/run/flatpak/at-spi-bus --env=AT_SPI_BUS_ADDRESS=unix:path=/run/flatpak/at-spi-bus --env=DESKTOP_SESSION=gnome --env=LANG=ru_RU.UTF-8 --env=WAYLAND_DISPLAY=wayland-0 --env=XDG_CURRENT_DESKTOP=GNOME --env=XDG_SESSION_DESKTOP=gnome --env=XDG_SESSION_TYPE=wayland --env=PATH=/app/bin:/app/bin:/app/bin:/app/bin:/usr/lib/sdk/vala/bin/:/app/bin:/usr/bin:/app/tools/podman/bin:/usr/lib/sdk/llvm17/bin:/usr/lib/sdk/rust-stable/bin:/usr/lib/sdk/typescript/bin:/usr/lib/sdk/node18/bin:/usr/lib/sdk/dotnet8/bin:/usr/lib/sdk/php83/bin:/var/config/composer/vendor/bin:/home/dell/.var/app/com.visualstudio.code/data/node_modules/bin:/app/bin:/usr/bin:/usr/lib/sdk/rust-stable/bin --env=LD_LIBRARY_PATH=/usr/lib/sdk/vala/lib/:/app/lib:/usr/lib/sdk/dotnet8/lib:/app/lib --env=PKG_CONFIG_PATH=/app/lib/pkgconfig:/app/share/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig --share=network --bind-mount=/run/host/fonts=/usr/share/fonts --bind-mount=/run/host/fonts-cache=/usr/lib/fontconfig/cache --filesystem=/home/dell/.local/share/fonts:ro --filesystem=/home/dell/.cache/fontconfig:ro --bind-mount=/run/host/user-fonts-cache=/home/dell/.cache/fontconfig --bind-mount=/run/host/font-dirs.xml=/home/dell/.cache/font-dirs.xml /home/dell/Projects/lemmy-reader/.flatpak/repo /usr/bin/meson "$@"