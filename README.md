<h3>A simple makefile generator for C programs written in Rust</h3>

<pre>
  <code>
    Enter Main File ex: main.c : 
    main.c
    Enter C Files seperated by spaces ex: 1.c 2.c : 
    1.c 2.c
    Enter Header File ex: main.h : 
    header.h
    ----------
    output: 2.o 1.o main.o
            gcc -Wall -std=c99 1.o 2.o main.c -o output
    1.o: 1.c header.h
            gcc -Wall -std=c99 -c 1.c
    2.o: 2.c header.h
            gcc -Wall -std=c99 -c 2.c
    main.o: main.c header.h
            gcc -Wall -std=c99 -c main.c
    clean:
            rm *.o output
    ----------
  </code>
</pre>
