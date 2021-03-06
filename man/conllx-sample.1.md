% CONLLX-SAMPLE(1)
% Daniel de Kok
% Feb 15, 2018

NAME
====

**conllx-sample** -- Take a random treebank sample

SYNOPSIS
========

**conllx-sample** *SAMPLE_SIZE* [*INPUT_FILE*] [*OUTPUT_FILE*]

DESCRIPTION
===========

The **conllx-sample** utility takes a random sample of size *SAMPLE_SIZE*
of a treebank using reservoir sampling.

If *INPUT_FILE* is not specified, **conllx-sample** will read from the
standard input. If *OUTPUT_FILE* is not specified, **conllx-sample** will
write from the standard output.

OPTIONS
=======

`-s SEED`

:    Initialize the random number generator with the given seed. Using a
     fixed seed makes the sampling reproducible.

SEE ALSO
========

conllx-cleanup(1),
conllx-compare(1),
conllx-cycle(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-tdz-expandmorph(1)
conllx-text(1),
