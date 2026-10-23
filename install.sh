
#!/bin/sh

# Dependências da minha WM (Penrose)

sudo pacman -Syu --needed \
    base-devel \
    pkgconf \
    xorg-server \
    xorg-xinit \
    xorg-xauth \
    xorg-xmodmap \
    xorg-setxkbmap \
    libx11 \
    libxcb \
    fontconfig \
    freetype2 \
    kitty \
    dmenu \
    feh \
    ttf-jetbrains-mono-nerd \
    pipewire \
    pipewire-pulse \
    pipewire-alsa \
    wireplumber \
    brightnessctl \
    mesa
