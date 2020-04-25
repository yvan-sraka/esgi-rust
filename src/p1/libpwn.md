`libpwn`: a not so complicated library to pwned a binary
========================================================

What about a lib that we could load dynamically (with `LD_PRELOAD`) that
has a custom, e.g., `malloc`, `random`, or `fork`?

![](https://imgs.xkcd.com/comics/random_number.png)

There is already a nice implementation of that:
<https://github.com/zardus/preeny>

This project is not so hard to implement and will help you to remind the
differences between what could be a syscall and what could be
implemented in a standard library.