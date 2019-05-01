
# rust-macros

Because this particular problem (how Rust macros work) caused me so much strife
in understanding how macros work in Rust, here's a sample package exhibiting
how to get macros to "escape" their scope. This "problem" (more generally a
poor understanding of the semantics by which Rust expands and keeps track
of macros currently "in scope") has been asked in various forms on stackoverflow:

- https://stackoverflow.com/questions/26388861/how-to-include-module-from-another-file-from-the-same-project
- https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
- https://stackoverflow.com/questions/29068716/how-do-you-use-a-macro-from-inside-its-own-crate

## An example

Now for the good part. The following is the output of running `src/main.rs`:

```bash
$ cargo run --bin main
...
Only reachable from main!
I'm ignoring you src/utils/expect.rs:masked_macro ! 7
src/bar/mod.rs:12] Expected 4 but got 3.
```

First, the `check_expect!` macro defined in `src/utils/expect.rs:2` "escapes" it's scope
and can be seen inside `src/bar/mod.rs:12` because of **both** of the `#[macro_use]`
annotations on the two module references in `src/main.rs:2` and `src/utils/mod.rs:2`
respectively. Namely the module reference in `src/main.rs:3` syntactically expands
out to the contents of `src/utils/mod.rs`, which itself expands out to a module
declaration containing the contents of `src/utils/expect.rs`.

But wait, if we delete **either** of the `#[macro_use]` lines in the aforementioned
files, we no longer have access to the `check_expect!` macro in `src/bar/mod.rs:12`.
This is because the Rust compiler (whatever version as of this writing) filters out
macros at *file* boundaries. If we take a closer look though, file boundaries don't
*actually* matter, it's module boundaries. Looking at `src/main2.rs`, we have a program
which is identical to `src/main.rs` but where the file hierarchy has been flattened
into one file.

In practice, this is exactly what the Rust compiler does: it runs a
preprocessing pass which expands the program's abstract syntax tree (AST) into
a single program / file unit. And again, if we delete either line
`src/main2.rs:2` or line `src/main2.rs:4`, the program will not compile because
the `check_expect!` macro will not be in scope on line `src/main2.rs:46`.
What this means in general is that macros defined inside a module declaration are
confined to that module *unless* the module itself is annotated with `#[macro_use]`.
With this annotation, macro definitions bleed *up* one level of module scope
(from module `expect` up and out into the `utils` module's scope). This process
then happens transitively for each level of a module tree for which you annotate
modules with `#[macro_use]`.

This process then, however, interacts with subsequent definitions of macros
by the same name resulting in shadowing. It seems that because macros are
handled at the AST / Rust tokens level, that Rust macros simply inhabit a
single namespace (the one that you are currently compiling). This means
that you can't use a syntax that looks like `module::macro!()` to invoke
a macro "inside" another module. It also means that a macro "flows through" a
file from top-to-bottom, when viewing the program in its expanded form as was
the case in our `src/main2.rs` file (and equivalently `src/main.rs`).

But as for macro shadowing: the `masked_macro` on line `src/main2.rs:20`
gets "shadowed" by a definition of the same name on line `src/main2.rs:29`
for two reasons. First, it needs to in scope on line 29 to even be considered
shadowed in the first place: which it is because of the `#[macro_use]` annotations
on lines 2 and 4. Secondly, it's shadowed because (1) it has the exact same name
(duh) and (2) that name is not qualified by `utils::expect::masked_macro!()` because
Rust does not support qualified references to macros (presumably because of
gritty details like macros being able to construct modules at compile time based
on the inputs to a macro invocation?).

But more importantly, and conceptually, the Rust compiler must maintain a
single set of Rust identifiers (macro names) each with their own corresponding
macro definition. When the Rust compiler then traipses upon a new declaration
of a macro with the *same name*, that newly named macro declaration *shadows*
the previous definition (at least until the new declaration goes out of scope
because we left a containing module without a `#[macro_use]` annotation, at which
point we must get back the old declaration in scope, but I haven't tested this).

## Language design

As someone who dabbles in various aspects of programming languages research and
who also happens to be largely unfamiliar with the intricacies of the semantics
for modules, namespaces, and macros in popular/mainstream programming
languages, I find it fascinating that Rust macros work the way they do.
No judgement intended here (full stop): it's curious that in order for a macro
to escape its scope you must explicitly annotate that module as such, but
in order to use a macro defined in an outer scope (and syntactically earlier
in the file-expansion process) macros implicitly fall into scope.

This is intuitive when compared in the light of scoping of local variables
across block statements in languages like C or Java, but only because local
variables in imperative languages that I'm familiar don't let the values
of local variables bleed out of block statements. But in the world of Rust
macros, we seem to care that macros can be invoked from a different sub-branch
of a program's AST (i.e. where module declarations are the AST edges we care
about), as evidenced by the fact that I wanted to invoke my `check_expect`
macro from a completely different module.

The current formulation of Rust macro semantics however, IMHO, leaves something
to be desired. Upon first encountering the problem for which I created this
git project, it wasn't immediately apparent to me how the annotation `#[macro_use]`
worked and how it interacted with other annotations like `#[macro_export]`
as well as parsing & macro expansion semantics like shadowing of macros and
implicit adoption of macros defined by syntactically previous modules when
the compiler sees a new module declaration. When I see shadowing like that
which is exhibited by this Rust package I see a blinking red sign that says
"danger ahead." This danger I envision could be ameliorated in one of (or all!)
three ways:

- Warn users when one macro is shadowing another macro (this seems to be
  the Rustic way of doing things, but I'm new here).
- Instrument the compiler with a debugging information which lets you query
  the macros in scope at a certain line of code. This could be e.g. a compiler
  flag like `--macros-in-scope src/main2.rs:37` which would tell you that `masked_macros`
  on line 29 and `check_expect` on line 6 are in scope, **and** that `masked_macros`
  on line 20 has been shadowed but will come back into scope after line 42.
- Take existing packages like the one presented herein, manually mutate them
  according to some principle (e.g. focusing on how macros work), and then
  give them to test subject programmers to accomplish some task. This could tell
  us when programmers are prone to introducing subtle bugs due to macro naming.

The first two ideas are concrete, while the third one is fairly ethereal. The
first idea is a quick patch, while the second idea is a more sustainable approach
which likely has the happy byproduct of lowering the learning curve on Rust macros
for new programmers.

