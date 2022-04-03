#! /bin/bash

mkdir $1

# Source file
echo "#include <iostream>" > $1/$1.cpp
echo -e "#include <my_utils.h>" >> $1/$1.cpp
echo -e "\n" >> $1/$1.cpp
echo -e "int main() {" >> $1/$1.cpp
echo -e "\tSolution solutions;" >> $1/$1.cpp
echo -e "}" >> $1/$1.cpp

# Makefile
echo "all: $1.cpp" > $1/Makefile
echo  -e "\tg++ -g -I.. $^ -o $1" >> $1/Makefile