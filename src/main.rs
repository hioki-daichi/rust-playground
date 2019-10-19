struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&'c self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

fn main() {
    let ctx = Context("foo");
    match parse_context(ctx) {
        Ok(_) => println!("{:?}", "ok"),
        Err(s) => println!("{:?}", s),
    }
}
