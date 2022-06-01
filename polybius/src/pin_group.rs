//! Logical groups of pins.

use embedded_hal::digital::v2::{InputPin, OutputPin};

/// A fixed-size group of output pins.
pub trait OutputGroup<const LEN: usize> {
    type Error;

    // NB: this is miscompiled on AVR targets, removing for now.
    //fn get(&mut self, index: usize) -> &mut dyn OutputPin<Error = Self::Error>;

    fn set_low(&mut self, index: usize) -> Result<(), Self::Error>;

    fn set_high(&mut self, index: usize) -> Result<(), Self::Error>;
}

/// A fixed-size group of input pins.
pub trait InputGroup<const LEN: usize> {
    type Error;

    // NB: same as `OutputGroup::get`
    //fn get(&self, index: usize) -> &dyn InputPin<Error = Self::Error>;

    fn is_low(&self, index: usize) -> Result<bool, Self::Error>;

    fn is_high(&self, index: usize) -> Result<bool, Self::Error>;
}

impl<T, const LEN: usize> OutputGroup<LEN> for [T; LEN]
where
    T: OutputPin,
{
    type Error = T::Error;

    /*
    fn get(&self, index: usize) -> &dyn OutputPin<Error = Self::Error> {
        &self[index]
    }
    */

    fn set_low(&mut self, index: usize) -> Result<(), Self::Error> {
        self[index].set_low()
    }

    fn set_high(&mut self, index: usize) -> Result<(), Self::Error> {
        self[index].set_high()
    }
}

impl<T, const LEN: usize> InputGroup<LEN> for [T; LEN]
where
    T: InputPin,
{
    type Error = T::Error;

    /*
    fn get(&mut self, index: usize) -> &mut dyn InputPin<Error = Self::Error> {
        &mut self[index]
    }
    */

    fn is_low(&self, index: usize) -> Result<bool, Self::Error> {
        self[index].is_low()
    }

    fn is_high(&self, index: usize) -> Result<bool, Self::Error> {
        self[index].is_high()
    }
}

macro_rules! tuple_impls {
    ($(
        ($t1:ident: $i1:tt, $($t:ident: $i:tt),*): $n:expr,
    )*) => {$(
        impl<$t1, $($t),*> OutputGroup<$n> for ($t1, $($t),*)
        where
            $t1: OutputPin,
            $($t: OutputPin<Error = $t1::Error>,)*
        {
            type Error = $t1::Error;

            /*
            fn get(&mut self, index: usize) -> &mut dyn OutputPin<Error = Self::Error> {
                match index {
                    $i1 => &mut self.$i1,
                    $( $i => &mut self.$i, )*
                    _ => panic!("index out of bounds"),
                }
            }
            */

            fn set_low(&mut self, index: usize) -> Result<(), Self::Error> {
                match index {
                    $i1 => self.$i1.set_low(),
                    $( $i => self.$i.set_low(), )*
                    _ => panic!("index out of bounds"),
                }
            }

            fn set_high(&mut self, index: usize) -> Result<(), Self::Error> {
                match index {
                    $i1 => self.$i1.set_high(),
                    $( $i => self.$i.set_high(), )*
                    _ => panic!("index out of bounds"),
                }
            }
        }

        impl<$t1, $($t),*> InputGroup<$n> for ($t1, $($t),*)
        where
            $t1: InputPin,
            $($t: InputPin<Error = $t1::Error>,)*
        {
            type Error = $t1::Error;

            /*
            fn get(&self, index: usize) -> &dyn InputPin<Error = Self::Error> {
                match index {
                    $i1 => &self.i1,
                    $( $i => &self.$i, )*
                    _ => panic!("index out of bounds"),
                }
            }
            */

            fn is_low(&self, index: usize) -> Result<bool, Self::Error> {
                match index {
                    $i1 => self.$i1.is_low(),
                    $( $i => self.$i.is_low(), )*
                    _ => panic!("index out of bounds"),
                }
            }

            fn is_high(&self, index: usize) -> Result<bool, Self::Error> {
                match index {
                    $i1 => self.$i1.is_high(),
                    $( $i => self.$i.is_high(), )*
                    _ => panic!("index out of bounds"),
                }
            }
        }
    )*};
}

tuple_impls! {
    (A: 0,): 1,
    (A: 0, B: 1): 2,
    (A: 0, B: 1, C: 2): 3,
    (A: 0, B: 1, C: 2, D: 3): 4,
    (A: 0, B: 1, C: 2, D: 3, E: 4): 5,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5): 6,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6): 7,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7): 8,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8): 9,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9): 10,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10): 11,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11): 12,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12): 13,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13): 14,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14): 15,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15): 16,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16): 17,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17): 18,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18): 19,
    (A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18, T: 19): 20,
}

#[cfg(test)]
mod tests {
    use super::*;

    use embedded_hal_mock::pin::{Mock, State, Transaction};

    #[test]
    fn output_group_0() {
        let mut pin_a = Mock::new(&[Transaction::set(State::High), Transaction::set(State::Low)]);
        let mut pin_b = Mock::new(&[]);
        let mut pin_c = Mock::new(&[]);

        let mut group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        group.set_high(0).unwrap();
        group.set_low(0).unwrap();

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }

    #[test]
    fn output_group_1() {
        let mut pin_a = Mock::new(&[]);
        let mut pin_b = Mock::new(&[Transaction::set(State::High), Transaction::set(State::Low)]);
        let mut pin_c = Mock::new(&[]);

        let mut group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        group.set_high(1).unwrap();
        group.set_low(1).unwrap();

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }

    #[test]
    fn output_group_2() {
        let mut pin_a = Mock::new(&[]);
        let mut pin_b = Mock::new(&[]);
        let mut pin_c = Mock::new(&[Transaction::set(State::High), Transaction::set(State::Low)]);

        let mut group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        group.set_high(2).unwrap();
        group.set_low(2).unwrap();

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }

    #[test]
    fn input_group_0() {
        let mut pin_a = Mock::new(&[Transaction::get(State::High), Transaction::get(State::Low)]);
        let mut pin_b = Mock::new(&[]);
        let mut pin_c = Mock::new(&[]);

        let group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        assert!(group.is_high(0).unwrap());
        assert!(group.is_low(0).unwrap());

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }

    #[test]
    fn input_group_1() {
        let mut pin_a = Mock::new(&[]);
        let mut pin_b = Mock::new(&[Transaction::get(State::High), Transaction::get(State::Low)]);
        let mut pin_c = Mock::new(&[]);

        let group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        assert!(group.is_high(1).unwrap());
        assert!(group.is_low(1).unwrap());

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }

    #[test]
    fn input_group_2() {
        let mut pin_a = Mock::new(&[]);
        let mut pin_b = Mock::new(&[]);
        let mut pin_c = Mock::new(&[Transaction::get(State::High), Transaction::get(State::Low)]);

        let group = (pin_a.clone(), pin_b.clone(), pin_c.clone());
        assert!(group.is_high(2).unwrap());
        assert!(group.is_low(2).unwrap());

        pin_a.done();
        pin_b.done();
        pin_c.done();
    }
}
