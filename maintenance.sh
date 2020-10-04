#!/bin/bash

HEIGHT=15
WIDTH=40
CHOICE_HEIGHT=4
BACKTITLE="Arch Linux Maintenance"
TITLE="Arch Linux Maintenance"
MENU="Choose one of the following options:"

OPTIONS=(1 "List orphan packages"
    2 "Remove orphan packages"
    3 "Remove cache"
4 "Exit")

while CHOICE=$(dialog --clear \
        --backtitle "$BACKTITLE" \
        --title "$TITLE" \
        --menu "$MENU" \
        $HEIGHT $WIDTH $CHOICE_HEIGHT \
        "${OPTIONS[@]}" \
    2>&1 >/dev/tty)
clear
do
    case $CHOICE in
        1)
            if [[ $(pacman -Qdtq) ]]; then
                echo "Orphan packages found:"
            else
                echo "No orphan packages found"
            fi
	    pacman -Qdtq
	    read -s -n 1 -p "Press any key to continue . . ."
            ;;
        2)
            sudo pacman -Rns $(pacman -Qdtq)
	    clear
	    echo "Orphans removed."
	    read -s -n 1 -p "Press any key to continue . . ."
            ;;
        3)
            sudo rm -rf ~/.cache/*
	    clear
	    echo "Cache removed."
	    read -s -n 1 -p "Press any key to continue . . ."
	    ;;
        4)
            break
            ;;
    esac
done
