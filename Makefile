out/main.wav: out/main
	c2t -bc8 out/main,6000 out/main.wav

out/main: main.c out/main.ll
	mos-clang $^ -o $@ --config mos-common.cfg -Os

out/main.ll: .FORCE
	mkdir -p out
	cargo rustc --release -- --emit=llvm-ir --crate-type=rlib -C opt-level=s
	cp target/release/deps/*.ll $@

clean:
	rm -rf out/
	cargo clean

.FORCE:
.PHONY: clean .FORCE
