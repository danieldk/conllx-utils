% CONLLX-CYCLE(1)
% Daniel de Kok
% Oct 29, 2017

NAME
====

**conllx-cycle** -- Find cycles in CoNLL-X dependency graphs

SYNOPSIS
========

**conllx-cycle** [*INPUT_FILE*]

DESCRIPTION
===========

The **conllx-cycle** utility detects cycles in CoNLL-X dependency graphs.
Self-cycles (a token that has itself as its head) are currently not
detected.

If *INPUT_FILE* is not specified, **conllx-cycle** will read from the
standard input.

OPTIONS
=======

`-p`

:    Find cycles in the projective column

SEE ALSO
========

conllx-cleanup(1),
conllx-compare(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-sample(1),
conllx-tdz-expandmorph(1)
conllx-text(1),
