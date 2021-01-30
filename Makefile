
TARGET_BINS = \
	target/release/bin-null-void \
	target/release/bin-plainerror \
	target/release/bin-std-error \
	target/release/bin-anyhow \
	target/release/bin-thiserror \
	target/release/bin-failure

all:

bench:
	cargo bench --no-run
	cargo bench --bench=bench-null-void  -- -n > target/z.bench-null-void.log
	cargo bench --bench=bench-plainerror -- -n > target/z.bench-plainerror.log
	cargo bench --bench=bench-thiserror  -- -n > target/z.bench-thiserror.log
	cargo bench --bench=bench-std-error  -- -n > target/z.bench-std-error.log
	cargo bench --bench=bench-anyhow     -- -n > target/z.bench-anyhow.log
	cargo bench --bench=bench-failure    -- -n > target/z.bench-failure.log
	size $(TARGET_BINS) > target/z.size-release.log

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

objdump:
	@env LANG=C objdump -d -j .text --demangle target/release/bin-null-void > target/z.objdump.null-void.txt
	@env LANG=C objdump -d -j .text --demangle target/release/bin-plainerror > target/z.objdump.plainerror.txt
	@env LANG=C objdump -d -j .text --demangle target/release/bin-std-error > target/z.objdump.std-error.txt
	@env LANG=C objdump -d -j .text --demangle target/release/bin-thiserror > target/z.objdump.thiserror.txt
	@env LANG=C objdump -d -j .text --demangle target/release/bin-anyhow > target/z.objdump.anyhow.txt
	@env LANG=C objdump -d -j .text --demangle target/release/bin-failure > target/z.objdump.failure.txt

report:
	cargo xtask shape_benchmark_results
