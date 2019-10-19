struct Context<'b>(&'b str);

struct Parser<'a, 'b> {
    context: &'a Context<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&'a self) -> Result<(), &'b str> {
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
