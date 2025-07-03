CC=clang


c: c/main.c
	${CC} -O3 -mllvm -polly -o out/queen_c c/main.c --static

go: go/main.go
	cd go && go build -o ../out/queen_go main.go

rust: rust/src/main.rs
	cd rust && cargo build --release
	@cp rust/target/release/queen out/queen_rust

all: c go rust

run: all
	@./out/queen_c $(n)
	@./out/queen_go $(n)
	@./out/queen_rust $(n)

# 用法: make run n=8