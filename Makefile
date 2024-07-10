APIS=$(shell find ../apis/bobadojo -name "*.proto")

all:	rpc 
	cargo build --all

clean:
	rm -rf cmd pkg


rpc:
	protoc ${APIS} -I ../apis --prost_out=src --tonic_out=src --prost-serde_out=src --prost-crate_out=gen_crate=Cargo.template:. 

