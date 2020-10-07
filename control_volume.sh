#!/bin/bash


#############################
# Simple script that controls
# volume with the amixer
# program
#############################

# The + option will increase volume
# The - option will decrease volume
# The m option will mute volume
# The M option will max out the volume
# Example : ./control volume +

# MUST HAVE amixer INSTALLED
# sudo pacman -S alsa-utils

if [ $# -ne 1 ]
then
	exit
fi

if [ $1 != '+' ] && [ $1 != '-' ] && [ $1 != 'm' ] && [ $1 != 'M' ]
then
	exit
fi

CURR_VOL=$(amixer -c 0 get Master | egrep -o '[0-9]?[0-9]?[0-9][ ][\[]' | egrep -o '[0-9]?[0-9]?[0-9]')

if [ $1 == '+' ]
then
	NEXT_VOL=$(expr $CURR_VOL + 5)
	amixer -c 0 set Master $NEXT_VOL
fi

if [ $1 == '-' ]
then
  NEXT_VOL=$(expr $CURR_VOL - 5)
	if [ $NEXT_VOL -lt 0 ]
	then
		NEXT_VOL=0
	fi
  amixer -c 0 set Master $NEXT_VOL
fi

if [ $1 == 'm' ]
then
	amixer -c 0 set Master 0
fi

if [ $1 == 'M' ]
then
	amixer -c 0 set Master 127
fi
