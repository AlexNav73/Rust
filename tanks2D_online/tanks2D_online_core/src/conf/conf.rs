
pub trait ConfCreator<T> {
    type Err;
    
    fn create(self) -> Result<T, Self::Err>; 
}
