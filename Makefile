



fmt:
	@@echo Formatting code...
	@@cargo fmt

clean:
	@@echo Cleaning up...
	@@cargo clean

.PHONY: clean fmt
