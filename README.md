# groundhog

A record and replay system

## Simple proof of concept

Currently in this repo there is a simple end to end example that will record traces in a C program with two child 
procs sending messages to each other using pipes. 

#### Running the record example

First, run `cargo build` to build the `.so` and header for our rust library in `groundhog-record`

Next, compile the c-example:
```
	gcc -L <ABSOLUTE-PATH>/groundhog/target/debug c-example/test.c -lgroundhog -o test
```

Run, the test binary
```
	LD_LIBRARY_PATH=<ABSOLUTE-PATH>/groundhog/target/debug ./test
```

This will now output two trace files, `node_a` and `node_b` which contains the traces from each child process 
in `test.c`

#### Creating a bundle

Now we can create a bundle file that will coalesce all of the trace files into one csv indexed by time
```
	./target/debug/gh-cli bundle node_a node_b
```

This will output a `bundle.gh` file

#### Replaying the bundle

All this will do is loop through the bundle.gh file and print each line


### Next Steps

The following toy example shows how we can build a simple c library that programs can call to log events they care 
about (e.g send and receive of messages). This library can store these logs and by storing the time these events
happened we can then replay these events for post-mortem debugging

To make this useful in real application here is what needs to be done:

- make the logger a much more efficient binary logger, currently it is just dumbly writing to a text file
- make the logger thread safe
- may want to use a time series database for the bundle file format for better searching
- make the replay tool actually follow the time stamps in the file, e.g if there is a second between two traces actually take 1 second
- make the replay tool output in a standaradized format (e.g: json via a web server) so that visualizers can consume the data (e.g a traffic graph to show messages passing between nodes/services)
