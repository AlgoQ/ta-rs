extern crate csv;
extern crate tars;

// TODO: implement some integration tests

#[cfg(test)]
mod test {
    #[cfg(feature = "serde")]
    mod serde {
        use tars::indicators::SimpleMovingAverage;
        use tars::Nexta;

        // Simple smoke test that serde works (not sure if this is really necessary)
        #[test]
        fn test_serde() {
            let mut macd = SimpleMovingAverage::new(20).unwrap();
            let bytes = bincode::serialize(&macd).unwrap();
            let mut deserialized: SimpleMovingAverage = bincode::deserialize(&bytes).unwrap();

            assert_eq!(deserialized.nexta(2.0), macd.nexta(2.0));
        }
    }
}
