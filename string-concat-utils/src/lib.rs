use itertools::Itertools;

pub fn concat_camel_case<S: AsRef<str>>(iter: impl IntoIterator<Item = S>) -> String {
    iter.into_iter()
        .map(|elem| {
            let str: &str = elem.as_ref();
            str.to_string()
        })
        .map(|word| word.to_lowercase())
        .map(|word| word.into_bytes())
        .map(|mut word| {
            if word.is_empty() {
                return "".to_string();
            }
            let (first, _) = word.split_at_mut(1);
            first[0] = char::from_u32(first[0] as u32)
                .unwrap()
                .to_uppercase()
                .next()
                .unwrap() as u8;
            String::from_utf8(word).unwrap()
        })
        .join("")
}
