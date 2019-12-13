# CoNLL-X Utilities

## Introduction

This is a set of utilities to modify files in the CoNLL-X tabular
files. The package contains the following programs:

* conllx-cleanup: replace most Unicode punctuation characters by
  by ASCII equivalents.
* conllx-compare: compare sentences on particular columns.
* conllx-cycle: find dependency trees with (non-self) cycles.
* conllx-grep: print sentences that have a token matching a pattern.
* conllx-merge: merge CoNLL-X files.
* conllx-partition: partition a CoNLL-X file in N files.
* conllx-sample: take a random sample from a CoNLL-X file.
* conllx-shuffle: shuffle sentences in a CoNLL-X file.
* conllx-text: convert CoNLL-X file to plain text.

## Download

Downloads are available on the [release
page](https://github.com/danieldk/conllx-utils/releases).

## Recent changes

* `conllx-tdz-expandmorph` has moved to the
  [TüBa-D/DP](https://github.com/sfb833-a3/tueba-ddp/tree/master/tools/general)
  tools, since it is corpus-specific.

## Usage

Executing a command gives usage information when `--help` is given
as an argument.

## Todo

A lot, including:

* Partitioning is currently interleaving. Also support chunked partitioning.
* Test with problematic inputs.
* Merge specific columns from two CoNLL files.
