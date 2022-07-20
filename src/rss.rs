pub fn list() -> String {
    return get_podcasts().join("\n\n")
}

fn get_podcasts() -> [&'static str; 4] {
    ["Test 1", "Test 2", "Blah", "Heyo"]
}