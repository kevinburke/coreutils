// spell-checker:ignore checkfile
macro_rules! get_hash(
    ($str:expr) => (
        $str.split(' ').collect::<Vec<&str>>()[0]
    );
);

macro_rules! test_digest {
    ($($id:ident $t:ident $size:expr)*) => ($(

    mod $id {
        use crate::common::util::*;
        static DIGEST_ARG: &'static str = concat!("--", stringify!($t));
        static BITS_ARG: &'static str = concat!("--bits=", stringify!($size));
        static EXPECTED_FILE: &'static str = concat!(stringify!($id), ".expected");
        static CHECK_FILE: &'static str = concat!(stringify!($id), ".checkfile");

        #[test]
        fn test_single_file() {
            let ts = TestScenario::new("hashsum");
            assert_eq!(ts.fixtures.read(EXPECTED_FILE),
                       get_hash!(ts.ucmd().arg(DIGEST_ARG).arg(BITS_ARG).arg("input.txt").succeeds().no_stderr().stdout_str()));
        }

        #[test]
        fn test_stdin() {
            let ts = TestScenario::new("hashsum");
            assert_eq!(ts.fixtures.read(EXPECTED_FILE),
                       get_hash!(ts.ucmd().arg(DIGEST_ARG).arg(BITS_ARG).pipe_in_fixture("input.txt").succeeds().no_stderr().stdout_str()));
        }

        #[test]
        fn test_check() {
            let ts = TestScenario::new("hashsum");
            ts.ucmd()
                .args(&[DIGEST_ARG, BITS_ARG, "--check", CHECK_FILE])
                .succeeds()
                .no_stderr()
                .stdout_is("input.txt: OK\n");
        }
    }
    )*)
}

test_digest! {
    md5 md5 128
    sha1 sha1 160
    sha224 sha224 224
    sha256 sha256 256
    sha384 sha384 384
    sha512 sha512 512
    sha3_224 sha3 224
    sha3_256 sha3 256
    sha3_384 sha3 384
    sha3_512 sha3 512
    shake128_256 shake128 256
    shake256_512 shake256 512
    b2sum b2sum 512
}
