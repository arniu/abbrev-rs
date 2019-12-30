use std::collections::HashMap;

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn abbrev<'a>(xs: &'a [&str]) -> HashMap<String, &'a str> {
    let space = " ";

    let mut result = HashMap::new();
    let mut sorted = xs.to_owned();

    sorted.sort();
    sorted.push(space); // append `space` to handle the last one
    sorted.windows(2).fold(space, |prev, curr_next| {
        let curr = curr_next[0];
        let next = curr_next[1];
        if *curr == *next {
            return curr;
        }

        let padding = space.repeat(diff(prev.len(), next.len()));
        let prev_chars = prev.chars().chain(padding.chars());
        let next_chars = next.chars().chain(padding.chars());
        let count = curr
            .chars()
            .zip(prev_chars.zip(next_chars))
            .scan((true, true), |(pe, ne), (c, (p, n))| {
                *pe = *pe && (c == p);
                *ne = *ne && (c == n);

                if *pe || *ne {
                    Some(true)
                } else {
                    None
                }
            })
            .count();

        let start = count + 1;
        for n in start..curr.len() {
            result.insert(curr.chars().take(n).collect(), curr);
        }
        // one can always be accessed by itself
        result.insert(curr.to_string(), curr);

        curr
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
