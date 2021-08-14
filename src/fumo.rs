pub const MESSAGE_INDICATOR: &str = "!message";

pub fn sayify(fumofile:  &mut String, message: &str) -> String {
    if fumofile.contains(MESSAGE_INDICATOR) {
        return fumofile.replace(MESSAGE_INDICATOR,message);
    }
    return format!("{}\n{}", fumofile, message);
}