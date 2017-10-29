% CONLLX-COMPARE(1)
% Daniel de Kok
% Mar 26, 2017

NAME
====

**conllx-compare** -- Compare CoNLL-X files on columns

SYNOPSIS
========

**conllx-compare** *INPUT_FILE* *INPUT_FILE2*

DESCRIPTION
===========

The **conllx-compare** compares two CoNLL-X files on certain columns. It
prints out sentences where one or more column values differ. The differing
columns are highlighted in red.

If one the extension of the file ends in *.gz*, the file is decompressed
while reading.

OPTIONS
=======

`-l LAYER,[LAYER2[,...]]`

:    Annotation layers to compare, default: *headrel*.

`-s LAYER,[LAYER2[,...]]`

:    Additional layers to print from the first file, default: *form*.

LAYERS
========

The possible layers for the `-l` and `-s` options are: *form*, *lemma*,
*cpos*, *pos*, *features*, *head*, *headrel*, *phead*, or *pheadrel*.

SEE ALSO
========

conllx-cycle(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-sample(1),
conllx-tdz-expandmorph(1)
conllx-text(1)
