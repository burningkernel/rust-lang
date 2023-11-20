#!/bin/bash

find ./ -maxdepth 3 -type d -name target -exec rm -rf {} \;

