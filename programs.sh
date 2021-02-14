#!/bin/sh
programs() {
    paru
    paru -S zsh telegram-desktop jetbrains-toolbox cmake visual-studio-code-bin discord timeshift mailspring yakuake
    curl -sS https://download.spotify.com/debian/pubkey_0D811D58.gpg | gpg --import -
    paru -S spotify
    sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}

programsyay() {
    pacman -S --needed git base-devel
    cd ~/Downloads
    git clone https://aur.archlinux.org/paru.git
    cd paru
    makepkg -si
    programs
    clear
}

if [[ $(paru) ]];
then
    if [[ $(programs) ]];
    then
        echo "All programs have been installed."
    else
        echo "Some programs did not install correctly."
    fi
else
    if [[ $(programsyay) ]];
    then
        echo "All programs have been installed."
    else
        echo "Some programs did not install correctly."
    fi
fi
