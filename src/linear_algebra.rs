pub mod matrix;
pub use matrix::Matrix2D;

trait MultiplicativeIdentityElement {
    fn mul_identity() -> Self;
}

impl MultiplicativeIdentityElement for u32 {
    fn mul_identity() -> Self {
        1
    }
} 

impl MultiplicativeIdentityElement for i64 {
    fn mul_identity() -> Self {
        1
    }
}
