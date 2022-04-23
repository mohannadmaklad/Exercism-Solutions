pub fn reply(message: &str) -> &str {
    let filterd_message: String = message
        .chars()
        .filter(|&c| c == '?' || c.is_ascii_alphabetic())
        .collect::<String>();

    let silence = message.trim().is_empty();
    let question = message.trim().ends_with('?');
    let shouting =
        filterd_message.to_uppercase() == filterd_message && filterd_message.len() > 1 as usize;

    match (silence, question, shouting) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, false) => "Sure.",
        (_, false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
