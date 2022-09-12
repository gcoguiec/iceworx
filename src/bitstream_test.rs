mod bitstream_struct_test {
    use std::io::Write;

    use crate::bitstream::{Bitstream, BitstreamOptions};

    #[test]
    fn it_reads_file_from_fs() {
        let temp_file = tempfile::Builder::new()
            .prefix("tmp")
            .suffix(&".bin".to_string())
            .rand_bytes(8)
            .tempfile()
            .unwrap();
        let content = "01234567".as_bytes();
        temp_file.as_file().write_all(content).unwrap();
        let path = temp_file.into_temp_path();
        let bitstream =
            Bitstream::new(path.to_str().unwrap(), BitstreamOptions::default())
                .unwrap();
        assert_eq!(bitstream.filesize, content.len());
        assert_eq!(bitstream.offset, 0);
    }

    mod debug_trait_test {
        use std::path::Path;

        use insta::assert_debug_snapshot;

        use crate::bitstream::Bitstream;

        #[test]
        fn it_renders_debug_representation() {
            let bitstream = Bitstream {
                filepath: Path::new("test.bin"),
                buffer: vec![48, 49, 50, 51],
                filesize: 4,
                offset: 5
            };
            assert_debug_snapshot!(bitstream, @r###"
            Bitstream {
                filepath: "test.bin",
                buffer: [
                    48,
                    49,
                    50,
                    51,
                ],
                filesize: 4,
                offset: 5,
            }
            "###);
        }
    }

    mod display_test {
        use std::path::Path;

        use insta::assert_debug_snapshot;

        use crate::{bitstream::Bitstream, CHUNK_SIZE};

        #[test]
        fn it_displays_bitstream_string_representation() {
            let bitstream = Bitstream {
                filepath: Path::new("test.bin"),
                buffer: vec![],
                filesize: CHUNK_SIZE,
                offset: 0
            };
            assert_debug_snapshot!(bitstream.to_string(),
            @r###""test.bin, bitstream size = 256""###);
        }
    }
}

mod chunk_struct_test {
    mod debug_trait_test {
        use insta::assert_debug_snapshot;

        use crate::bitstream::Chunk;

        #[test]
        fn it_renders_debug_representation() {
            let chunk = Chunk(0, 0..2, false);
            assert_debug_snapshot!(chunk, @r###"
            Chunk(
                0,
                0..2,
                false,
            )
            "###);
        }
    }
}

mod chunk_iter_test {
    use std::path::Path;

    use crate::{
        bitstream::{Bitstream, Chunk},
        CHUNK_SIZE
    };

    #[test]
    fn it_iterates_over_bitstream_address_chunks() {
        let bitstream = Bitstream {
            filepath: Path::new("test.bin"),
            buffer: vec![],
            filesize: 2 * CHUNK_SIZE,
            offset: 0
        };
        let chunks: Vec<Chunk> = bitstream.chunk_iter().collect();
        assert_eq!(chunks[0], Chunk(0, 0..CHUNK_SIZE, false));
        assert_eq!(
            chunks[1],
            Chunk(CHUNK_SIZE, CHUNK_SIZE..2 * CHUNK_SIZE, true)
        );
    }
}
