currenttime=$(date +%H:%M)
while true
do
	if [[ "$currenttime" > "18:00" ]] || [[ "$currenttime" < "07:00" ]]; then
		if grep -q 'default' plasmarc; then
			lookandfeeltool -a org.kde.breezedark.desktop
		fi
	else
		if grep -q 'breeze-dark' plasmarc; then
			lookandfeeltool -a org.kde.breeze.desktop
		fi
	fi
done
