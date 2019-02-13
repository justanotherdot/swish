swish
=====

> deterministically compare sets of file

# Why?

Sometimes you want to know if two sets of files are exactly the same but you do not want to care about finding some hacky way of checking them. Consider two JSON files, for example, that may be validly sorted by the specification, but may not compare easily. One could sort the files and compare them using `diff` but this approach isn't easily reproducible to every file that may not necessarily be line-delimited.

# How?

`git` compares packfiles using a SHA1 of the sorted collection of the SHA1s for every git object. swish does the same thing, effectively, but extrapolates it out to two or more files as need be.
