use std::collections::HashMap;

pub fn abbrev<'a>(xs: &'a [&str]) -> HashMap<String, &'a str> {
    let mut result = HashMap::new();
    let mut sorted = xs.to_owned();

    sorted.sort();
    sorted.push(""); // append "" to handle the last one
    sorted.windows(2).fold(0, |matched, curr_next| {
        let curr = curr_next[0];
        let next = curr_next[1];
        if *curr == *next {
            return matched;
        }

        let matches = curr
            .chars()
            .zip(next.chars())
            .take_while(|(c, n)| c == n)
            .count();

        let max = matches.max(matched);
        for n in (max + 1)..curr.len() {
            result.insert(curr.chars().take(n).collect(), curr);
        }

        // one can always be accessed by itself
        result.insert(curr.to_string(), curr);

        matches
    });

    result
}

#[cfg(test)]
mod tests {
    use abbrev;

    #[test]
    fn it_works() {
        let xs = vec!["foo", "fool", "folding", "flop"];
        let map = abbrev(&xs);

        for key in vec!["fl", "flo", "flop"] {
            assert_eq!(&"flop", map.get(key).unwrap());
        }

        for key in vec!["fol", "fold", "foldi", "foldin", "folding"] {
            assert_eq!(&"folding", map.get(key).unwrap());
        }

        for key in vec!["foo"] {
            assert_eq!(&"foo", map.get(key).unwrap());
        }

        for key in vec!["fool"] {
            assert_eq!(&"fool", map.get(key).unwrap());
        }

        assert_eq!(10, map.len());
    }
}
