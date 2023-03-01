pub fn ass () {
    let mut noun = String::new();
    let mut verb = String::new();
    let mut adjective = String::new();
    let mut adverb = String::new();
    let fnoun = nouns(noun).trim().to_string();
    let fverb = verbs(verb).trim().to_string();
    let fadjective = adjectives(adjective).trim().to_string();
    let fadverb = adverbs(adverb).trim().to_string();
    print!("\n do you {} your {} {} {}, That's hilarious!", fverb, fadjective, fnoun, fadverb);
}

fn nouns (mut noun : String) -> String {
    println!("Enter a noun:");
    std::io::stdin().read_line(&mut noun);
    return noun

}

fn verbs (mut verb : String) -> String {
    println!("Enter a verb:");
    std::io::stdin().read_line(&mut verb);
    return verb

}

fn adjectives (mut adjective : String) -> String {
    println!("Enter a adjective:");
    std::io::stdin().read_line(&mut adjective);
    return adjective

}


fn adverbs (mut adverb : String) -> String {
    println!("Enter a adverb: :");
    std::io::stdin().read_line(&mut adverb);
    return adverb

}

