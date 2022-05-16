#! /bin/bash

mkdir $1

# Source file
echo "package $1;" > $1/$1.java
echo "class Main {" >> $1/$1.java
echo -e "\tpublic static void main(String args[]) {" >> $1/$1.java
echo -e "\t\tSystem.out.println(\"Stuff\");" >> $1/$1.java
echo -e "\t\tSolution solution = new Solution();" >> $1/$1.java
echo -e "\t}" >> $1/$1.java
echo -e "}" >> $1/$1.java

# Makefile
echo "all: $1.java" > $1/Makefile
echo  -e "\tjavac -g $^" >> $1/Makefile
