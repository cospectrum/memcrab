use memcrab_cache::MemLru;

#[test]
fn builder() {
    let max_len = 10;
    let max_bytesize = 50;

    let memlru = MemLru::<String, Vec<u8>>::builder()
        .max_len(max_len)
        .max_bytesize(max_bytesize)
        .build();

    assert_eq!(memlru.max_bytesize(), max_bytesize);
    assert_eq!(memlru.max_len(), max_len);

    let memlru = MemLru::<String, Vec<u8>>::builder()
        .max_bytesize(max_bytesize)
        .build();

    assert_eq!(memlru.max_bytesize(), max_bytesize);
    assert_eq!(memlru.max_len(), usize::MAX);
}
