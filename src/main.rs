fn main() {
    let words = vec!["rust", "java", "typescript", "csharp", "caa", "rustc"];
    let mut sorted = sort(words, vec![score, bonus, penalty]);
    sorted.reverse();
    println!("Sorted words: {:?}", sorted);
}

fn sort(words: Vec<&str>, functions: Vec<fn(&str) -> isize>) -> Vec<&str> {
    let mut sorted_words = words.clone();

    let aggregated = aggregate(functions);

    sorted_words.sort_by(|a, b| aggregated(a).cmp(&aggregated(b)));
    sorted_words
}

fn aggregate(functions: Vec<fn(&str) -> isize>) -> impl Fn(&str) -> isize {
    move |word: &str| -> isize { functions.iter().map(|f| f(word)).sum() }
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