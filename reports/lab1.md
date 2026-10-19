# Lab
This lab is about implementing ``sys_trace`` in the kernel.
The ``sys_trace`` is not only about "trace", but read and write.
I guess this design is referring to the behaviour of ``ptrace``, but maybe cause consfusion for beginners.
## My implementation
It's sad to know we cannot use a hash map directly without standard library. So I simply use the syscall number as the key. It can cause memmory waste but it's easy and useful.
