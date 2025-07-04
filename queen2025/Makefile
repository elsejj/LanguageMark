CC=clang
n ?= 8

c: c/main.c out
	${CC} -O3 -mllvm -polly -o out/queen_c c/main.c --static

go: go/main.go out
	cd go && go build -o ../out/queen_go main.go

rust: rust/src/main.rs out
	cd rust && cargo build --release
	@cp rust/target/release/queen out/queen_rust

out:
	mkdir -p out

all: c go rust

run: all
	@./out/queen_c $(n)
	@./out/queen_go $(n)
	@./out/queen_rust $(n)

# 用法: make run n=8