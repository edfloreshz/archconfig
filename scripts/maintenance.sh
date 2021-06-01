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
    4 "List failed services"
    5 "Open kernel journal"
    6 "Full system upgrade"    
    7 "Exit")

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
            ;;
        2)
            sudo pacman -Rns $(pacman -Qdtq)
            clear
            echo "Orphans removed."
            ;;
        3)
	    echo "$(du -sh ~/.cache/) will be removed"
	    sudo rm -rf ~/.cache/*
            clear
	    echo "Cache removed."
            ;;
        4)
            systemctl --failed
	    ;;
        5)
            sudo journalctl -p 3 -xb
            ;;
	6)
	    sudo pacman -Syu
	    ;;	   	    
	7)
	    break
	    ;;
    esac
    read -s -n 1 -p "Press any key to continue . . ."
done
