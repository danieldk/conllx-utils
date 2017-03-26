% CONLLX-CLEANUP(1)
% Daniel de Kok
% Sep 21, 2016

NAME
====

**conllx-cleanup** -- Clean up CoNLL-X tokens

SYNOPSIS
========

**conllx-cleanup** [*INPUT_FILE*] [*OUTPUT_FILE*]

DESCRIPTION
===========

The **conllx-cleanup** utility replaces non-ASCII unicode punctuation
characters by ASCII equivalents (where possible). This generally improves
processing using tools that have not seen non-ASCII punctuation characters
in their training data.

If *INPUT_FILE* is not specified, **conllx-cleanup** will read from the
standard input. If *OUTPUT_FILE* is not specified, **conllx-cleanup** will
write to the standard output.

SEE ALSO
========

conllx-compare(1),
conllx-cycle(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-sample(1),
conllx-tdz-expandmorph(1)
conllx-text(1)
