pub trait FluxValue<T> {
    fn get_flux_value(height: u32) -> T;
}

pub struct FluxChangeValue<T> {
    pub(crate) _height: u32,
    pub(crate) _new_value: T,
}
impl<T> FluxChangeValue<T> {
    pub fn new(_height: u32, _new_value: T) -> Self {
        Self {
            _height,
            _new_value,
        }
    }
}

pub struct Flux<T> {
    _genesis_value: T,
    /// A vec of tuple of u32 (height) and T (the value)
    _changes: Vec<FluxChangeValue<T>>,
}
impl<T> Flux<T> {
    pub const fn new(genesis_value: T, changes: Vec<FluxChangeValue<T>>) -> Self {
        Self {
            _genesis_value: genesis_value,
            _changes: changes,
        }
    }
    pub fn get_flux_value(_height: u32) -> T {
        todo!()
    }
}

//pub static MAX_PAYLOAD_LENGTH: Flux<u32> = Flux::<u32>::new(
//    255 * 176,
//    vec![
//        FluxChangeValue::new(historical_moments::PRE_POC2.get().unwrap(), 255 * 176 * 4),
//        FluxChangeValue::new(
//            historical_moments::SMART_FEES_ENABLE.get().unwrap(),
//            255 * (176 + 8) * 4 * 2,
//        ),
//    ],
//);
//pub static MAX_PAYLOAD_LENGTH: Flux<u32> = Flux::<u32>::new(
//    255 * 176,
//    vec![
//        FluxChangeValue::new(historical_moments::PRE_POC2.get().unwrap(), 255 * 176 * 4),
//        FluxChangeValue::new(
//            historical_moments::SMART_FEES_ENABLE.get().unwrap(),
//            255 * (176 + 8) * 4 * 2,
//        ),
//    ],
//);
