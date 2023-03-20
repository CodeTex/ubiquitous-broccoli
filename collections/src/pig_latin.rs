pub fn convert_to(string: &String) -> String{
    let suffix = "hay";
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    // word starts with vowels
    if vowels.iter().any(|v| string.starts_with(*v)) {
        return format!("{string}-{suffix}")
    }

    format!("{}-{}{}", &string[1..], &string[0..1], &suffix[1..])
}
