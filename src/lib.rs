pub mod first;
pub mod second;

#[allow(dead_code)]
fn try_first_list() {
    let _ = first::List::new();
}

#[cfg(test)]
mod tests {
}
