#!/bin/bash
mkdir cache
mkdir ../models
nextflow run train_tagger.nf --outputdir $(realpath ../models) --model bert-base-dutch-cased --traindata inputdata/multi-ner-nld.train.txt --devdata inputdata/sonar1-ner-nld.dev.txt --testdata inputdata/sonar1-ner-nld.test.txt --name bert-ner-cased-multi-nld --cache_dir $(realpath cache) --examplespath $(realpath transformers/examples)  -with-trace
