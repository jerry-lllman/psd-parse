use psd_rs::PSD;

#[test]
fn load_psd() {
  let psd = include_bytes!("./fixtures/check-font.psd");

  let psd = PSD::from_bytes(psd).unwrap();

  dbg!(psd);
}