Time to complete: ~3.5 hours
Speed-optimized runtime: <=80ms (`perf stat [PROG]`)
Speed-optimized max heap: 19kB (`valgrind --trace-children=yes --tool=massif [PROG] && ms_print massif.out.* | head -40`)
Size-optimized executable size: 333kB
