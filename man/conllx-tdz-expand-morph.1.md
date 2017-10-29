% conllx-tdz-expand-morph(1)
% Daniel de Kok
% Oct 29, 2017

NAME
====

**conllx-tdz-expand-morph** -- Expand Tueba-D/Z morphology annotations

SYNOPSIS
========

**conllx-tdz-expand-morph** [*OPTIONS*] [*INPUT_FILE*] [*OUTPUT_FILE*]

DESCRIPTION
===========

The **conllx-tdz-expand-morph** utility expands the short morphology tags in
the TüBa-D/Z to attribute-value pairs. For example, the tag

    3sit

is expanded to:

    person:3|number:singular|mood:indicative|tense:past|morph:3sit

The original short tag is included as the *morph* feature.

If *INPUT_FILE* is not specified, **conllx-tdz-expand-morph** will read from
the standard input. If *OUTPUT_FILE* is not specified,
**conllx-tdz-expand-morph** will write from the standard output.

OPTIONS
=======

`-n`

:    Do not add include the original tag as a feature.

SEE ALSO
========

conllx-cleanup(1),
conllx-compare(1),
conllx-cycle(1),
conllx-grep(1),
conllx-merge(1),
conllx-partition(1),
conllx-sample(1),
conllx-text(1),
