ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/myliblog

target:
	mkdir -p $@

target/myliblog: target/myliblog.o
	$(CC) -o $@ $^ $(LDFLAGS)

target/myliblog.o: myliblog.c | target
	$(CC) -o $@ -c $<

.PHONY : clean
clean :
	rm -rf target