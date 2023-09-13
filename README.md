# fdate - a TUI calendar for scripting

I could not find a text-only, shell TUI calender widget that I could shove into
shell pipelines to let the user (=myself) input dates.

So here's fdate, a command line date picker. 

Shows three months, on 'enter', pushes the date to stdout and returns exit code
0. Exit code 1 on 'escape/ctrl-c'. Perfect to use in shell scripts. 

Date can be preset, just pass YYYY-MM-DD on command line.

Decent keyboard navigation. No mouse input. Can go next/last "day of the
week"/week/month/year with a single keypress. Numeric input of arbitrary dates.
Use 'tab' to skip year/month. Doesn't allow you to type invalid dates, such as
the 31st of February, or the 13th month. One press keys for today (.) tomorrow
(>), yesterday (<), default date (,).

Can be used to show data from the selected date (using `--search-command`). Can
highlight dates (`--highlight=YYYY-MM-DD`).  Can show an arbitrary title / input string (`--title=...`).
Pass `--help` for all options.

Uses ISO formated dates (YYYY-mm-dd) exclusively. Shows (ISO)
week number. Years go over 9000. The myriannum ends on a Friday, apperantly.

![fdate demonstration gif](./docs/demo.gif)
