# nap
nap is a suspend-aware alternative to sleep for shell scripts etc. It ensures 
the program doesn't wait longer than necessary, immediately terminating if the
system was suspended during the sleep period and the requested time has passed.

# usage
```
Usage: nap [OPTIONS] [NUMBER]

Arguments:
  [NUMBER]
          Pause for <NUMBER> of seconds. SUFFIX may be:
            - 's' for seconds (default)
            - 'm' for minutes
            - 'h' for hours
            - 'd' for days
          Specify the time to pause with the optional suffix. If no suffix 
          is provided, seconds is assumed.

Options:
      --midnight
          Pause the timer when it reaches midnight.

      --at <at>
          Set a specific time to pause the timer (e.g., 18:00 or 6:00PM). You 
          can use either 24-hour or 12-hour format.

      --sleep
          Just do what sleep does

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
