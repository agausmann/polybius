use embedded_hal::digital::v2::OutputPin;

/// An output pin that can additionally be set to floating / high-impedance.
pub trait TriStatePin {
    type Error;

    /// Drives the pin low.
    fn set_low(&mut self) -> Result<(), Self::Error>;

    /// Drives the pin high.
    fn set_high(&mut self) -> Result<(), Self::Error>;

    /// Disconnects the driver, setting the pin floating.
    fn set_floating(&mut self) -> Result<(), Self::Error>;
}

/// A tri-state adapter for pins whose modes are reflected as typestate.
pub enum Typestate<T>
where
    T: IntoFloating,
{
    Output(T),
    Floating(T::FloatingPin),

    /// A temporary value swapped-in while converting between modes.
    ///
    /// This should never be seen in practice, but it is necessary to satisfy
    /// the borrow checker when we take ownership of the pin to convert it,
    /// because we always have to leave ourselves in a valid state.
    ///
    /// You can read more about why this is necessary here:
    /// http://smallcultfollowing.com/babysteps/blog/2018/11/10/after-nll-moving-from-borrowed-data-and-the-sentinel-pattern/
    Uninit,
}

impl<T> Typestate<T>
where
    T: IntoFloating,
{
    fn into_output(self) -> Self {
        match self {
            Self::Floating(floating) => Self::Output(T::from_floating(floating)),
            _ => self,
        }
    }

    fn into_floating(self) -> Self {
        match self {
            Self::Output(output) => Self::Floating(T::into_floating(output)),
            _ => self,
        }
    }
}

impl<T> TriStatePin for Typestate<T>
where
    T: OutputPin + IntoFloating,
{
    type Error = T::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        *self = core::mem::replace(self, Self::Uninit).into_output();
        match self {
            Self::Output(output) => output.set_low(),
            _ => unreachable!(),
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        *self = core::mem::replace(self, Self::Uninit).into_output();
        match self {
            Self::Output(output) => output.set_high(),
            _ => unreachable!(),
        }
    }

    fn set_floating(&mut self) -> Result<(), Self::Error> {
        *self = core::mem::replace(self, Self::Uninit).into_floating();
        Ok(())
    }
}

pub trait IntoFloating {
    type FloatingPin;

    fn into_floating(self) -> Self::FloatingPin;

    fn from_floating(floating: Self::FloatingPin) -> Self;
}
