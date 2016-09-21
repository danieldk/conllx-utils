% CONLLX-cycle(1)
% Daniël de Kok
% Sep 21, 2016

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

The following options are available:

`-p`

:    Find cycles in the projective column

SEE ALSO
========

conllx-cleanup(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-sample(1),
conllx-tdz-expandmorph(1)
