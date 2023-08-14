all: tcp_server tcp_client

%: %.rs
	rustc -g $@.rs

clean:
	rm -rf pointer_test vec_stack r_dag ref_cell lifetime generic_read \
		buf_builder animal clone_trait copy_trait allocator borrow_test \
		slice_test deref_test sizeof_fn tcp_server tcp_client
