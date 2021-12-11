fn main() {
    let ctx_lines = 2;
    let needle = "00";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let tags: Vec<usize> = vec![];
    let ctx: Vec<Vec<usize, String>> = vec![];

    for (i, line) in haystack.lines.enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }
}
