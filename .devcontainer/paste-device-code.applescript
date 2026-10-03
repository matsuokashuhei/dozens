tell application "System Events"
	set activeBrowser to first application process whose frontmost is true
	tell activeBrowser
		keystroke "v" using {command down}
	end tell
end tell
