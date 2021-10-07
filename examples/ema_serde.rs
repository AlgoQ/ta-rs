use tars::indicators::ExponentialMovingAverage as Ema;
use tars::DataItem;
use tars::Nexta;

fn main() {
    let mut ema = Ema::new(9).unwrap();
    let mut reader = csv::Reader::from_path("./examples/data/AMZN.csv").unwrap();

    for record in reader.deserialize() {
        // Deserialize DataItem but ignore the `date` field.
        // You may have to create your own struct if you want to keep track of the date.
        let dt: DataItem = record.unwrap();
        let ema_val = ema.nexta(&dt);
        println!("{} = {:2.2}", ema, ema_val);
    }
}
