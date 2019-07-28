#!/bin/sh
set -e

SAMPLE_DIR=samples
URAND_FILE=$SAMPLE_DIR/urandom
TEXT_FILE=$SAMPLE_DIR/text

echo "Samples builder for Ranaz, populating '$SAMPLE_DIR'"

rm -Rf $SAMPLE_DIR
mkdir $SAMPLE_DIR
dd if=/dev/urandom of=$URAND_FILE count=1024 2>/dev/null

cat /etc/init.d/* > $TEXT_FILE

echo -n "LZMA\t"
xz -1 -c $URAND_FILE > ${URAND_FILE}_xz1
xz -1 -c $TEXT_FILE > ${TEXT_FILE}_xz1
xz -9 -c $URAND_FILE > ${URAND_FILE}_xz9
xz -9 -c $TEXT_FILE > ${TEXT_FILE}_xz9
echo "OK"

echo -n "ZIP\t"
zip ${URAND_FILE}_zip9 -9 $URAND_FILE >/dev/null
zip ${TEXT_FILE}_zip9 -9 $TEXT_FILE > /dev/null
zip ${URAND_FILE}_zip1 -1 $URAND_FILE > /dev/null
zip ${TEXT_FILE}_zip1 -1 $TEXT_FILE > /dev/null
echo "OK"

echo -n "GUNZIP\t"
gzip -9 -c $URAND_FILE > ${URAND_FILE}_gzip9
gzip -9 -c $TEXT_FILE > ${TEXT_FILE}_gzip9
gzip -1 -c $URAND_FILE > ${URAND_FILE}_gzip1
gzip -1 -c $TEXT_FILE > ${TEXT_FILE}_gzip1
echo "OK"
