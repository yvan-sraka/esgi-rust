#! /usr/bin/env python3
import os, sys
paths = []
for arg in sys.argv:
    for path, subdirs, files in os.walk(arg):
        for name in files:
            paths.append(os.path.join(path, name))
hashcat = ""
for f in sorted(paths):
    hashcat += os.popen("md5sum %s" % f).read().split()[0]
os.system("echo \"%s\" | md5sum" % hashcat)
