#[derive(Debug)]
pub enum OutputCommand<'a> {
    Create { backend: &'a str, name: &'a str },
    Remove { name: &'a str },
}

#[derive(Debug)]
pub enum SetErrorCommand<'a> {
    Set { color: &'a str, message: &'a str },
    Disable,
}

#[derive(Debug)]
pub enum DismissProps {
    All,
    Total(u32),
}
