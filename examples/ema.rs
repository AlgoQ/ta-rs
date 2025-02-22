use tars::indicators::ExponentialMovingAverage as Ema;
use tars::DataItema;
use tars::Nexta;

fn main() {
    let mut ema = Ema::new(9).unwrap();
    let mut reader = csv::Reader::from_path("./examples/data/AMZN.csv").unwrap();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();
        let dt = DataItema::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();
        let ema_val = ema.nexta(&dt);
        println!("{}: {} = {:2.2}", date, ema, ema_val);
    }
}
