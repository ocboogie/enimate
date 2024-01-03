use crate::building::Builder;

pub trait Component {
    type Handle;
    fn build(&self, builder: &mut impl Builder) -> Self::Handle;
}
