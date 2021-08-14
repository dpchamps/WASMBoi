pub enum JumpConditionError {
    InvalidLookupInput,
}
type JumpConditionValue = &'static str;
type JumpConditionLookupResult = Result<JumpConditionValue, JumpConditionError>;

pub const NZ: JumpConditionValue = "NZ";
pub const Z: JumpConditionValue = "Z";
pub const NC: JumpConditionValue = "NC";
pub const C: JumpConditionValue = "C";

pub fn lookup_jump_condition(input: u8) -> JumpConditionLookupResult {
    match input {
        0b00 => Ok(NZ),
        0b01 => Ok(Z),
        0b10 => Ok(NC),
        0b11 => Ok(C),
        _ => Err(JumpConditionError::InvalidLookupInput),
    }
}
