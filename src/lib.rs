pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if is_silence(trimmed_message) {
        "Fine. Be that way!"
    } else if is_question(trimmed_message) {
        if is_yelling(trimmed_message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if is_yelling(trimmed_message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    message == message.to_uppercase() && message.chars().any(|c| c.is_alphabetic())
}
