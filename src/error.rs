use crate::string::SString;
use crate::ERROR_LEN;

#[derive(Debug)]
pub enum Error
{
    CollectionsError(SString<ERROR_LEN>)
}