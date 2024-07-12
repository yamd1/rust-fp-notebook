fn main() {
    let words = vec!["ada", "haskell", "scala", "java", "rust"];
    let mut sorted = sort(words, vec![score, bonus, penalty]);
    sorted.reverse();
    println!("Sorted words: {:?}", sorted);

    let aggregate = aggregate(vec![score, bonus, penalty].clone());

    let func = high_scoreing_words(sorted, aggregate);
    println!("high score words: {:?}", func(1));
}

fn sort(words: Vec<&str>, functions: Vec<fn(&str) -> isize>) -> Vec<&str> {
    let mut sorted_words = words.clone();

    let aggregated = aggregate(functions);

    sorted_words.sort_by(|a, b| aggregated(a).cmp(&aggregated(b)));
    sorted_words
}

fn aggregate<'a>(functions: Vec<fn(&'a str) -> isize>) -> impl Fn(&'a str) -> isize + Clone + 'a {
    move |word: &'a str| -> isize { functions.iter().map(|f| f(word)).sum() }
}

fn score(word: &str) -> isize {
    word.replace("a", "").len() as isize
}

fn bonus(word: &str) -> isize {
    if word.contains("c") {
        return 5;
    }
    0
}

fn penalty(word: &str) -> isize {
    if word.contains("s") {
        return -7;
    }
    0
}

fn high_scoreing_words<'a, F>(words: Vec<&'a str>, function: F) -> impl Fn(isize) -> Vec<&'a str>
where
    F: Fn(&'a str) -> isize + Clone + 'a,
{
    move |threshold: isize| -> Vec<&'a str> {
        words
            .clone()
            .into_iter()
            .filter(|&word| function(word) > threshold)
            .collect()
    }
}
