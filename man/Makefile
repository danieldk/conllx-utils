all: conllx-cleanup.1 conllx-compare.1 conllx-cycle.1

clean:
	rm -f *.html

web: conllx-cleanup.1.html conllx-cycle.1.html

%.1: %.1.md
	pandoc -s -w man -o $@ $<

%.1.html: %.1.md
	pandoc -s -c manpage.css -w html5 -o $@ $<
