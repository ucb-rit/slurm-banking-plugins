#!/bin/bash

BASEDIR=$(dirname $0)
NOW=$(date +%s)
DAYS_AGO=1
START_DATE=$(($NOW-86400*$DAYS_AGO))

NOW_DATE_STRING=$(date --date="@$NOW" +%F)
START_DATE_STRING=$(date --date="@$START_DATE" +%F)
$BASEDIR/../show-jobs-raw.sh $START_DATE_STRING $NOW_DATE_STRING | $BASEDIR/jobs2rest.py
