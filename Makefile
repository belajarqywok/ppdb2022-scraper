ds  = datasets
ext = ppdb2022.exe

build :
	cargo build

run :
	mv ./target/debug/$(ext) .
	$(ext)
	rm -irf $(ext)

drop_datasets :
	rm -irf $(ds)

compress_datasets :
	tar -a -c -f $(ds).zip $(ds)

notebook :
	python3 -m notebook

start : build run compress_datasets drop_datasets