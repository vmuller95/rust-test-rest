#!/bin/bash

curl -X PUT --form "fileupload=@test-images/test1.jpg" --form "fileupload=@test-images/test2.jpg" 127.0.0.1:9999/load_image