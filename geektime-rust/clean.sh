#!/bin/bash

find ./ -maxdepth 2 -type d -name target -exec rm -rf {} \;

