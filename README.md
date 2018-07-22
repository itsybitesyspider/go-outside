go-outside
----------

If, like me, you live in an area where there's sometimes a spot of lovely weather in the middle of a long run of horribleness, and you don't want to miss it, then you might appreciate this.

Scriptable utility to assess the current whether from the command line.

go-outside doesn't just report the weather -- it assesses whether or not the weather is good.

### Examples:

Report the current weather, right now:

	go-outside --coordinates LAT LON --now

Report the single best weather over the comming forecast:

	go-outside --coordinates LAT LON --best

Report the current weather only if it also the best: 

	go-outside --coordinates LAT LON --now --best

For example, make a bash script that only runs when the weather is good:

	if [ -z "$(go-outside --now --best)" ];
		echo "The weather is good right now!"
	fi

Or, put this into your cron to get a regular graphical pop-up only when the weather is particularly good:

	notify-send "$(go-outside --now --best)"
