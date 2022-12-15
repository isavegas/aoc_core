fn get_input(year: usize, day: usize) -> Result<String, ()> {
    let mut dir = std::env::home_dir()?;
    dir.push(".cache");
    dir.push("aoc");
    dir.push(year.to_string());
    dir.push(day.to_string());
    fs::create_dir_all(&dir)?;
}
