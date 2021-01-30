#[cfg(test)]
mod test {
    use cmp_error::*;
    //
    #[test]
    fn test_sz() {
        assert_eq!(std::mem::size_of::<MyError>(), 4);
        assert_eq!(std::mem::size_of::<ThisError>(), 4);
        assert_eq!(std::mem::size_of::<Box<dyn std::error::Error>>(), 16);
        assert_eq!(std::mem::size_of::<anyhow::Error>(), 8);
        assert_eq!(std::mem::size_of::<failure::Error>(), 16);
        //
        assert_eq!(std::mem::size_of::<std::io::Result<()>>(), 16);
        assert_eq!(std::mem::size_of::<std::io::Result<u8>>(), 24);
        assert_eq!(std::mem::size_of::<std::io::Result<u16>>(), 24);
        assert_eq!(std::mem::size_of::<std::io::Result<u32>>(), 24);
        assert_eq!(std::mem::size_of::<std::io::Result<u64>>(), 24);
        assert_eq!(std::mem::size_of::<std::io::Result<u128>>(), 24);
        //
        assert_eq!(std::mem::size_of::<Result<u16, ()>>(), 4);
        assert_eq!(std::mem::size_of::<Result<u16, MyError>>(), 8);
        assert_eq!(std::mem::size_of::<Result<u16, ThisError>>(), 8);
        assert_eq!(std::mem::size_of::<Result<u16, anyhow::Error>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u16, Box<dyn std::error::Error>>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u16, failure::Error>>(), 24);
        //
        assert_eq!(std::mem::size_of::<Result<u32, ()>>(), 8);
        assert_eq!(std::mem::size_of::<Result<u32, MyError>>(), 8);
        assert_eq!(std::mem::size_of::<Result<u32, ThisError>>(), 8);
        assert_eq!(std::mem::size_of::<Result<u32, anyhow::Error>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u32, Box<dyn std::error::Error>>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u32, failure::Error>>(), 24);
        //
        assert_eq!(std::mem::size_of::<Result<u64, ()>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u64, MyError>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u64, ThisError>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u64, anyhow::Error>>(), 16);
        assert_eq!(std::mem::size_of::<Result<u64, Box<dyn std::error::Error>>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u64, failure::Error>>(), 24);
        //
        assert_eq!(std::mem::size_of::<Result<u128, ()>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u128, MyError>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u128, ThisError>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u128, anyhow::Error>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u128, Box<dyn std::error::Error>>>(), 24);
        assert_eq!(std::mem::size_of::<Result<u128, failure::Error>>(), 24);
    }
}
