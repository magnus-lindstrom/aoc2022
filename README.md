# Fetching data from website

You need to store your session cookie from the aoc website in a `.session_cookie` file. In Chrome
(or Brave), find it under "Inspect (F12) > Application > Storage > Cookies > session".

When you have saved your session cookie, simply run the script `get_daily_input.sh` with a numerical
input. For day X in December, give X as input. The input file will be stored under `inputs/X.txt`.
