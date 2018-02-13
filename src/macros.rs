#[macro_export]
macro_rules! impl_binop {
    (impl $trait: ident $method: ident for $type: ty) => {
        impl<'a> $trait<&'a $type> for $type {
            type Output = $type;

            fn $method(self, rhs: &'a $type) -> Self::Output {
                (&self).$method(rhs)
            }
        }

        impl<'a> $trait<$type> for &'a $type {
            type Output = $type;

            fn $method(self, rhs: $type) -> Self::Output {
                self.$method(&rhs)
            }
        }

        impl $trait<$type> for $type {
            type Output = $type;

            fn $method(self, rhs: $type) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_scalar {
    (impl $trait: ident $method: ident for $type: ty) => {
        impl<'a> $trait<&'a $type> for f32 {
            type Output = $type;

            fn $method(self, rhs: &'a $type) -> Self::Output {
                rhs.$method(self)
            }
        }

        impl $trait<$type> for f32 {
            type Output = $type;

            fn $method(self, rhs: $type) -> Self::Output {
                (&rhs).$method(self)
            }
        }

        impl $trait<f32> for $type {
            type Output = $type;

            fn $method(self, rhs: f32) -> Self::Output {
                (&self).$method(rhs)
            }
        }
    };
}
