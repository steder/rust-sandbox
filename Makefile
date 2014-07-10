all: hello

hello: hello.rs
	rustc $<

clean:
	rm -f hello
