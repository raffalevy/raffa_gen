#!/bin/bash

clang src/setup.m -c -o setup.o -fmodules -fobjc-arc
ar -rcs libsetup.a setup.o