#!/bin/bash

test_image1=`cat test-images/test1-base64.txt`
test_image2=https://ih1.redbubble.net/image.608339956.2125/flat,750x,075,f-pad,750x1000,f8f8f8.jpg
#echo  
curl -H "Content-Type: application/json" --data "[{\"upload_type\": \"base64\", \"param\": \"$test_image1\"}, {\"upload_type\": \"uri\", \"param\": \"$test_image2\"}]" \
    http://127.0.0.1:9999/load_image