

/// I am portable
pub struct Portable;

#[doc(cfg(unix))]
pub mod unix_only {
    // @has doc_cfg/unix_only/fn.unix_only_function.html \
    //  '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' \
    //  'This is supported on Unix only.'
    // @count - '//*[@class="stab portability"]' 1
    pub fn unix_only_function() {
        content::should::be::irrelevant();
    }

    // @has doc_cfg/unix_only/trait.ArmOnly.html \
    //  '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' \
    //  'This is supported on Unix and ARM only.'
    // @count - '//*[@class="stab portability"]' 1
    #[doc(cfg(target_arch = "arm"))]
    pub trait ArmOnly {
        fn unix_and_arm_only_function();
    }

    #[doc(cfg(target_arch = "arm"))]
    impl ArmOnly for super::Portable {
        fn unix_and_arm_only_function() {}
    }
}
