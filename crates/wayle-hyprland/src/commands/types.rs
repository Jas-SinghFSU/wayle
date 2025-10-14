pub enum OutputCommand<'a> {
    Create { backend: &'a str, name: &'a str },
    Remove { name: &'a str },
}

pub enum SetErrorCommand<'a> {
    Set { color: &'a str, message: &'a str },
    Disable,
}

pub enum DismissProps {
    All,
    Total(u32),
}
