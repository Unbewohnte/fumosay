/// Indicator of where the message should be in fumofile
pub const MESSAGE_INDICATOR: &str = "!message";
/// Fumofile name of the default fumo
pub const FUMO_DEFAULT: &str = "fumo.fumo"; 

/// Returns a resulting string with `MESSAGE_INDICATOR` replaced with given
/// `message`. If `MESSAGE_INDICATOR` is not present in fumofile - the
/// `message` will be added on the new line at the end of the fumofile. 
pub fn sayify(fumofile_contents:  &mut String, message: &str) -> String {
    if fumofile_contents.contains(MESSAGE_INDICATOR) {
        return fumofile_contents.replace(MESSAGE_INDICATOR,message);
    }
    return format!("{}\n{}", fumofile_contents, message);
}