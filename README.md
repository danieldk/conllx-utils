# CoNLL-X Utilities

## Introduction

This is a set of utilities to modify files in the CoNLL-X tabular files. The
focus of this package is interoperability with TCF (Text Corpus Format).
However, the majority of the utilities are also useful outside TCF. The
package contains the following programs:

* conllx-cleanup: replace most Unicode punctuation characters by
  by ASCII equivalents.
* conllx-compare: compare sentences on particular columns.
* conllx-cycle: find dependency trees with (non-self) cycles.
* conllx-tdz-expandmorph: expand morphology features in TüBa-D/Z.
* conllx-tdz-reattach-aux-pps: re-attach topicalized PPs in TüBa-D/Z
  to the main/content verb.
* conllx-grep: print sentences that have a token matching a pattern.
* conllx-merge: merge CoNLL-X files.
* conllx-partition: partition a CoNLL-X file in N files.
* conllx-sample: take a random sample from a CoNLL-X file.
* conllx-text: convert CoNLL-X file to plain text.

## Download

Downloads are available on the [release
page](https://github.com/danieldk/conllx-utils/releases).

## Usage

Executing a command gives usage information when `--help` is given
as an argument.

## Todo

A lot, including:

* Partitioning is currently interleaving. Also support chunked partitioning.
* Test with problematic inputs.
* Merge specific columns from two CoNLL files.
