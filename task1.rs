const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

fn main() {
    if let Some(result) = find_term(SEARCH_TERM, QUOTE) {
        println!("{}", result);
    } else {
        println!("The search term was not found in the quote.");
    }
}

fn find_term(search_term: &str, quote: &str) -> Option<String> {
    for (line_number, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            return Some(format!("{}: {}", line_number + 1, line));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);
        assert_eq!(Some("2: dark square is a picture feverishly turned--in search of what?".to_string()), answer);
    }
}