#[derive(Debug)]
pub struct Context<'a>(pub &'a str);

#[derive(Debug)]
pub struct Parser<'a, 'c> {
    ctx: &'a Context<'c>,
}

impl<'a, 'c> Parser<'a, 'c> {
    pub fn echo(&self) -> &'c str {
        self.ctx.0
    }
}

pub(crate) fn parse_context<'a>(cntx: &Context<'a>) -> &'a str {
    Parser { ctx: cntx }.echo()
}

pub(crate) fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
