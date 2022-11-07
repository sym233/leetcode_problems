# Read from the file words.txt and output the word frequency list to stdout.

(tr " " "\n" | sed "/^\s*$/d" | sort | uniq -c | sort -bnr | sed "s/\s\+\([0-9]\+\) \([a-z]\+\)/\2 \1/") < words.txt
