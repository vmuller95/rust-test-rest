#!/bin/bash

test_image1=`cat test-images/test1-base64.txt`
#echo  
curl --data-urlencode "upload_types[1]=base64" --data-urlencode "params[1]=$test_image1"  http://127.0.0.1:9999/load_image