#![cfg(test)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
extern crate brotli_decompressor;
extern crate core;
use std::io::Write;

use super::brotli::concat::{BroCatli, BroCatliResult};
use super::brotli::enc::BrotliEncoderParams;
use super::integration_tests::UnlimitedBuffer;
use brotli_decompressor::{CustomRead, CustomWrite};
const RANDOM_THEN_UNICODE: &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
const ALICE: &'static [u8] = include_bytes!("../../testdata/alice29.txt");
const UKKONOOA: &'static [u8] = include_bytes!("../../testdata/ukkonooa");
const ASYOULIKE: &'static [u8] = include_bytes!("../../testdata/asyoulik.txt");
const BACKWARD65536: &'static [u8] = include_bytes!("../../testdata/backward65536");
const DICTWORD: &'static [u8] = include_bytes!("../../testdata/ends_with_truncated_dictionary");
const RANDOM10K: &'static [u8] = include_bytes!("../../testdata/random_org_10k.bin");
const RANDOMTHENUNICODE: &'static [u8] = include_bytes!("../../testdata/random_then_unicode");
const QUICKFOX: &'static [u8] = include_bytes!("../../testdata/quickfox_repeated");
const EMPTY: &'static [u8] = &[];
use super::Rebox;

const BYTE_ALIGNED_START_BLOCK_WINDOW_22: &'static [u8] = &[0x6b, 0x00];
const BYTE_ALIGNED_END_BLOCK: &'static [u8] = &[0x3];

fn concat(
    files: &mut [UnlimitedBuffer],
    brotli_files: &mut [UnlimitedBuffer],
    byte_aligned_brotli_files: &mut [UnlimitedBuffer],
    window_override: Option<u8>,
    bs: usize,
) {
    let mut obuffer = vec![0u8; bs];
    let mut brotli_ibuffer = vec![0u8; bs];
    let mut bytealigned_ibuffer = Vec::with_capacity(
        bs + BYTE_ALIGNED_START_BLOCK_WINDOW_22.len() + BYTE_ALIGNED_END_BLOCK.len(),
    );
    let mut ooffset = 0usize;
    let mut ioffset;
    let mut uboutput = UnlimitedBuffer::new(&[]);

    // concat byte-aligned if files are provided
    {
        bytealigned_ibuffer
            .write_all(BYTE_ALIGNED_START_BLOCK_WINDOW_22)
            .unwrap();

        for (file, brotli) in files.iter_mut().zip(byte_aligned_brotli_files.iter_mut()) {
            // test decompressing the indiviudal block
            {
                let mut block_ibuffer = UnlimitedBuffer::new(&[]);
                block_ibuffer
                    .write_all(BYTE_ALIGNED_START_BLOCK_WINDOW_22)
                    .unwrap();
                block_ibuffer.write_all(brotli.data()).unwrap();
                block_ibuffer.write_all(BYTE_ALIGNED_END_BLOCK).unwrap();

                let mut rt = UnlimitedBuffer::new(&[]);
                match super::decompress(&mut block_ibuffer, &mut rt, 65536, Rebox::default()) {
                    Ok(_) => {}
                    Err(e) => panic!("Error {:?}", e),
                }
                assert_eq!(&rt.data(), &file.data());
            }

            // byte concat the compressed bare block
            bytealigned_ibuffer.write_all(&brotli.data()).unwrap();
        }

        bytealigned_ibuffer
            .write_all(BYTE_ALIGNED_END_BLOCK)
            .unwrap();

        let mut bytealigned_ibuffer = UnlimitedBuffer::new(&bytealigned_ibuffer);
        let mut rt = UnlimitedBuffer::new(&[]);
        match super::decompress(&mut bytealigned_ibuffer, &mut rt, 65536, Rebox::default()) {
            Ok(_) => {}
            Err(e) => panic!("Error {:?}", e),
        }
        let mut offset = 0;
        for file in files.iter_mut() {
            file.reset_read();
            assert_eq!(&rt.data()[offset..offset + file.data().len()], file.data());
            offset += file.data().len();
        }
        assert_eq!(offset, rt.data().len());
    }

    // concat normal
    {
        let mut output = super::IoWriterWrapper(&mut uboutput);
        let mut bro_cat_li = match window_override {
            Some(ws) => BroCatli::new_with_window_size(ws),
            None => BroCatli::new(),
        };
        for brotli in brotli_files.iter_mut() {
            bro_cat_li.new_brotli_file();
            {
                let mut input = super::IoReaderWrapper(brotli);
                loop {
                    ioffset = 0;
                    match input.read(&mut brotli_ibuffer[..]) {
                        Err(e) => panic!("{}", e),
                        Ok(cur_read) => {
                            if cur_read == 0 {
                                break;
                            }
                            loop {
                                match bro_cat_li.stream(
                                    &brotli_ibuffer[..cur_read],
                                    &mut ioffset,
                                    &mut obuffer[..],
                                    &mut ooffset,
                                ) {
                                    BroCatliResult::NeedsMoreOutput => {
                                        match output.write(&obuffer[..ooffset]) {
                                            Err(why) => panic!("couldn't write: {:}", why),
                                            Ok(count) => {
                                                assert_eq!(count, ooffset);
                                            }
                                        }
                                        ooffset = 0;
                                    }
                                    BroCatliResult::NeedsMoreInput => {
                                        break;
                                    }
                                    BroCatliResult::Success => {
                                        panic!("Unexpected state: Success when streaming before finish");
                                    }
                                    failure => {
                                        panic!("{:?}", failure);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            brotli.reset_read();
        }
        loop {
            match bro_cat_li.finish(&mut obuffer[..], &mut ooffset) {
                BroCatliResult::NeedsMoreOutput => {
                    match output.write(&obuffer[..ooffset]) {
                        Err(why) => panic!("couldn't write\n{:}", why),
                        Ok(count) => {
                            assert_eq!(count, ooffset);
                        }
                    }
                    ooffset = 0;
                }
                BroCatliResult::NeedsMoreInput => {
                    panic!("Unexpected EOF");
                }
                BroCatliResult::Success => {
                    if ooffset != 0 {
                        match output.write(&obuffer[..ooffset]) {
                            Err(why) => panic!("couldn't write\n{:}", why),
                            Ok(count) => {
                                assert_eq!(count, ooffset);
                            }
                        }
                    }
                    break;
                }
                failure => {
                    panic!("{:?}", failure)
                }
            }
        }
    }

    let mut rt = UnlimitedBuffer::new(&[]);
    match super::decompress(&mut uboutput, &mut rt, 65536, Rebox::default()) {
        Ok(_) => {}
        Err(e) => panic!("Error {:?}", e),
    }
    let mut offset = 0;
    for file in files {
        file.reset_read();
        assert_eq!(&rt.data()[offset..offset + file.data().len()], file.data());
        offset += file.data().len();
    }
    assert_eq!(offset, rt.data().len());
}

fn concat_many_subsets(
    files: &mut [UnlimitedBuffer],
    brotli_files: &mut [UnlimitedBuffer],
    bytealign_brotli_files: &mut [UnlimitedBuffer],
    window_override: Option<u8>,
) {
    let test_plans: [(usize, usize); 4] =
        [(brotli_files.len(), 4096 * 1024), (4, 1), (3, 3), (2, 4096)];
    for plan_bs in test_plans.iter() {
        let files_len = files.len();
        for index in 0..(brotli_files.len() - core::cmp::min(plan_bs.0 - 1, files_len)) {
            let file_subset = &mut files[index..core::cmp::min(index + plan_bs.0, files_len)];
            let brotli_subset =
                &mut brotli_files[index..core::cmp::min(index + plan_bs.0, files_len)];
            let bytealign_subset =
                &mut bytealign_brotli_files[index..core::cmp::min(index + plan_bs.0, files_len)];
            concat(
                file_subset,
                brotli_subset,
                bytealign_subset,
                window_override,
                plan_bs.1,
            );
        }
    }
}

#[cfg(debug_assertions)]
fn light_debug_test(params: &mut BrotliEncoderParams) {
    params.quality = 5;
}

#[cfg(not(debug_assertions))]
fn light_debug_test(_params: &mut BrotliEncoderParams) {}

#[cfg(debug_assertions)]
fn medium_debug_test(params: &mut BrotliEncoderParams) {
    params.quality = 9;
    params.q9_5 = false;
}

#[cfg(not(debug_assertions))]
fn medium_debug_test(_params: &mut BrotliEncoderParams) {}

#[test]
#[should_panic]
fn test_appendonly_twice_fails() {
    let mut files = [
        UnlimitedBuffer::new(UKKONOOA),
        UnlimitedBuffer::new(QUICKFOX),
    ];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = true;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();
    }
    concat(
        &mut files[..],
        &mut ufiles[..],
        &mut byte_aligned_ufiles,
        None,
        2,
    );
}

#[test]
fn test_append_then_empty_works() {
    let mut files = [UnlimitedBuffer::new(UKKONOOA), UnlimitedBuffer::new(&[])];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(
        &mut files[..],
        &mut ufiles[..],
        &mut byte_aligned_ufiles[..],
        None,
        2,
    );
}

#[test]
fn test_append_then_cat_works() {
    let mut files = [
        UnlimitedBuffer::new(UKKONOOA),
        UnlimitedBuffer::new(QUICKFOX),
    ];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_one_byte_works() {
    let mut files = [UnlimitedBuffer::new(UKKONOOA), UnlimitedBuffer::new(&[8])];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_one_byte_before_works() {
    let mut files = [UnlimitedBuffer::new(&[8]), UnlimitedBuffer::new(UKKONOOA)];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_two_byte_works() {
    let mut files = [
        UnlimitedBuffer::new(UKKONOOA),
        UnlimitedBuffer::new(&[8, 9]),
    ];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_two_byte_before_works() {
    let mut files = [
        UnlimitedBuffer::new(&[8, 9]),
        UnlimitedBuffer::new(UKKONOOA),
    ];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);
        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_empty_then_cat_works() {
    let mut files = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(QUICKFOX)];
    let mut ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut byte_aligned_ufiles = [UnlimitedBuffer::new(&[]), UnlimitedBuffer::new(&[])];
    let mut first = true;
    let buffers = files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut());
    for ((src, dst), align_dst) in buffers {
        let mut params0 = BrotliEncoderParams::default();
        params0.appendable = first;
        params0.catable = !first;
        params0.use_dictionary = first;
        super::compress(src, dst, 4096, &params0, &[], 1).unwrap();

        src.reset_read();
        let mut params0 = BrotliEncoderParams::default();
        byte_align_params(&mut params0);

        super::compress(src, align_dst, 4096, &params0, &[], 1).unwrap();

        first = false;
    }
    concat(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None, 2);
}

#[test]
fn test_concat() {
    let mut files = [
        UnlimitedBuffer::new(ALICE),
        UnlimitedBuffer::new(RANDOMTHENUNICODE),
        UnlimitedBuffer::new(UKKONOOA),
        UnlimitedBuffer::new(ASYOULIKE),
        UnlimitedBuffer::new(BACKWARD65536),
        UnlimitedBuffer::new(EMPTY),
        UnlimitedBuffer::new(DICTWORD),
        UnlimitedBuffer::new(RANDOM10K),
        UnlimitedBuffer::new(QUICKFOX),
    ];
    let mut params0 = BrotliEncoderParams::default();
    light_debug_test(&mut params0);
    let mut params1 = params0.clone();
    params1.quality = 9;
    params1.q9_5 = true;
    params1.lgwin = 22;
    params1.magic_number = true;
    medium_debug_test(&mut params1);
    let mut params2 = params0.clone();
    params2.quality = if params1.q9_5 || params1.quality > 9 {
        9
    } else {
        8
    };
    params2.lgwin = 19;
    let mut params3 = params0.clone();
    params3.quality = 8;
    params3.lgwin = 16;
    params3.magic_number = true;
    let mut params4 = params0.clone();
    params4.quality = 7;
    params4.lgwin = 14;
    let mut params4 = params0.clone();
    params4.quality = 1;
    params4.lgwin = 10;
    let mut params5 = params0.clone();
    params5.quality = 0;
    params5.lgwin = 10;
    params5.magic_number = true;
    params0.lgwin = 26;
    params0.large_window = true;

    let mut options = [params0, params1, params2, params3, params4, params5];
    for option in options.iter_mut().skip(3) {
        let mut ufiles = [
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
        ];
        let mut byte_aligned_ufiles = [
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
            UnlimitedBuffer::new(&[]),
        ];
        let mut first = true;
        for ((src, dst), align_dst) in files
            .iter_mut()
            .zip(ufiles.iter_mut())
            .zip(byte_aligned_ufiles.iter_mut())
        {
            let mut block_align_option = option.clone();

            if first {
                option.appendable = true;
            } else {
                option.appendable = false;
                option.catable = true;
                option.use_dictionary = false;
            }
            super::compress(src, dst, 4096, option, &[], 1).unwrap();

            src.reset_read();
            byte_align_params(&mut block_align_option);
            super::compress(src, align_dst, 4096, &block_align_option, &[], 1).unwrap();

            src.reset_read();
            first = false;
        }
        concat_many_subsets(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None);
        return;
    }
    let mut ufiles = [
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
    ];
    let mut byte_aligned_ufiles = [
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
        UnlimitedBuffer::new(&[]),
    ];
    let options_len = options.len();
    for (index, ((src, dst), align_dst)) in files
        .iter_mut()
        .zip(ufiles.iter_mut())
        .zip(byte_aligned_ufiles.iter_mut())
        .enumerate()
    {
        let option = &mut options[core::cmp::min(index, options_len - 1)];
        option.catable = true;
        option.use_dictionary = false;
        option.appendable = false;
        option.quality = core::cmp::max(2, option.quality);
        // ^^^ there's an artificial limitation of using 18 as the minimum window size for quality 0,1
        // since this test depends on different window sizes for each stream, exclude q={0,1}
        super::compress(src, dst, 4096, option, &[], 1).unwrap();

        src.reset_read();
        let mut block_align_option = option.clone();
        byte_align_params(&mut block_align_option);
        super::compress(src, align_dst, 4096, &block_align_option, &[], 1).unwrap();

        src.reset_read();
    }
    concat_many_subsets(&mut files, &mut ufiles, &mut byte_aligned_ufiles, None);
    concat_many_subsets(&mut files, &mut ufiles, &mut byte_aligned_ufiles, Some(28));
    // FIXME: make this 28
}

fn byte_align_params(option: &mut BrotliEncoderParams) {
    option.byte_align = true;
    option.bare_stream = true;
    option.use_dictionary = false;
    option.catable = true;
}
