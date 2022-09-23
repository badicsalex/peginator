## Peginator syntax

The syntax is similar to [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form),
except for small additions, the lack of a sequence marker, and `|` used for ordered choice
instead of `/`.

### Rules

A peginator grammar is a list of rules.

Rules look like this:

```ebnf
RuleName = expression;
```

Rule names are directly used as Rust type names, so they shouldn't conflict with standard
type names, keywords, or the Peginator built-in names.

Use PascalCase for rule names, if you can. Fully upper-case names are not special.

You can prefix rules with various [directives](#directives), the most important being
[@export](#export).

### Expressions

#### `expression1 expression2`

**Sequence**: To match two or more expression parts one after another. If any of the expressions fail
to match, the whole sequence expression will fail.

#### `expression1 | expression2`

**Ordered choice**: it will try to match expression1, and if it was unsuccessful,
it will try to match expression2. If none of them were successful, the choice expression
will be unsuccessful too.

**Precedence between choice and sequence**: Choice has the least precedence of all expressions, i.e. these two rules are the same:

```ebnf
Rule1 =  'a' 'b'  |  'c' 'd'
Rule1 = ('a' 'b') | ('c' 'd')
```

#### `[expression]`

**Optional**: Try to match the expression, but not fail if it does not match. If half the
expression matches, and there are field matches in there, the fields will not
be updated.

#### `{expression}`

**Closure**: Match an expression zero or more times, greedily.

#### `{expression}+`

**Positive closure**: Match an expression one or more times, greedily.

#### `!expression`

**Negative lookahead**: fail if expression matches, succeed and don't consume input if it doesn't.

It may not contain any field declarations.

It has higher precedence than sequences and choices, so these two rules are the same:

```ebnf
Rule1 = a !b c;
Rule2 = a (!b) c;
```

Mainly useful to preserve the last element of a sequentially matched pattern:

```ebnf
Rule = first:Item {!(Item $) middle:Item} last:Item;
```

Or to match any character but one:
```ebnf
Rule = '(' {!')' body:char} ')';
```

#### `&expression`

**Positive lookahead**: succeeds if expression matches, doesn't consume input.

It may not contain any field declarations.

Precedence is the same as the Negative lookahead.

#### `(expression)`

**Group**: match the expression. Used to clarify precedences.

#### `'string'` or `"string"`

**String or character literal**. There is no difference between the quote styles. If there
is only a single character between the quotes, an optimized character match
will be generated.

The following escape sequences are supported:
* `\n`, `\r`, `\t`, `\\`, `\'`, `\"`
* `\xXX` (converts to unicode between 0x80 and 0xFF)
* `\uXXXX` (exactly 4 hex digits)
* `\U00XXXXXX` (Exactly 8 hex digits, the first two must be `0`)
* `\u{XXXX}` (1 to 6 hex digits)

#### `'a'..'z'`

**Character range** (inclusive).

The same escape sequences work as for strings.

#### `$`

**End of input**: fail if there are any unparsed characters left.

#### Rule match

Rules can be matched without recording their output, by simply using their name:

```ebnf
RuleName
```

This will match the rule, but not put it into any field, basically dropping any structure
it may contain.

There is no automatic functionality of e.g. putting these results in a tuple in peginator.

A special rule type is `char`, which matches exactly one utf-8 character.

#### Fields

Fields, parts of the generated Rule `struct`s are written as part of any expression:

```ebnf
field_name:RuleName
```

This will create a field with `field_name`, and type `FieldType`. During parsing, the
`FieldType` rule will be matched.

To `Box` the field, prefix the type name with `*` (useful if there is a cycle in the types, or the matched field is rare
and big):

```ebnf
boxed_field:*BigRule
```

If a field is optional in the grammar (e.g. it's part of an optional, or it
does not exist on all choice alternatives), it will be encapsulated in an
`Option`.

If a field appears multiple times (in the same choice arm) or is part of a
closure, it will be encapsulated in a `Vec`. In this case `Box`-ing it is not
needed, and will not be done even if `*` is used.

If a field appears multiple times with different types, an `enum` type with the various
choices will be created and automatically used.

Fields deep in the rule expressions will be flattened into the rule `struct` itself:

```ebnf
Rule = 'something' [ ':' { element:ElementRule ',' }+ ]
```

```ignore
struct Rule {
    element:Vec<ElementRule>,
}
```

`char` can be used as a field type, in this case the field will have the type `char`, and
will match exactly one utf-8 character.

#### Override

Override declarations are similar to actual field declarations:
```ebnf
@:RuleType
```

This will match `RuleType`, and will directly assign the result to the Rule itself.

Override declarations can appear with multiple different types on different arms of
choices. In this case, the Rule will become an `enum` declaration:

```ebnf
EnumRule = @:Type1 | @:Type2 | 'prefix' @:Type3;
```

Overrides must appear exactly once on all arms of choice declarations, and cannot be
part of optional or closure construct.

`char` can be used as the type, in this case the rule will have the type `char`.

### Directives

#### `@export`

Exports the rule as a parse root, i.e. implements the [`PegParser`] traits on it.

#### `@no_skip_ws`

Disables whitespace skipping for the rule. It does not disable skipping in the internals
of matched rules.

#### `@position`

Record the start and end positions (byte indexes) of the rule match in the Rule `struct`.

#### `@string`

Force the rule to be a `String`. All field declarations will be ignored, and the whole match
will be recorded.

This is functionality is meant to replace the use of regular expression matches.

Should be used with `@no_skip_ws`

#### `@char`

Force the rule to be a `char`. These are special kind of rules, which shall not have any other
directives, and can only contain a single choice, where all arms are simple characters, character
ranges or char rule matches.

Useful for defining character classes:
```ebnf
@char
Hexadecimal = 'a'..'f' | 'A'..'F' | '0'..'9';

@char
IdentifierChar =  Hexadecimal | '_';
```

#### `@memoize`

Enable memoization for the rule. It has a non-trivial performance impact (good or bad), so only use
when it's actually needed. (Mainly when having multiple alternative rules with the same, complex
prefix and different postfix)

#### `@check(...)`

Add additional checks to a rule. The "parameter" of the `@check` directive is a function name that
will be called with the result of the rule. The function name should be fully qualified (i.e. start
with a crate name or `crate::`).

The function shall return a bool. True means the check was successful.

Multiple checks can be added to a single rule.

For example, the following rule:

```ebnf
@check(crate::checks::check_point)
Point = '(' x:Number ',' y:Number ')'
```

Will call the following function in the `checks` module:

```ignore
pub fn check_point(p: &Point) -> bool {
    ...
}
```

### `@extern(...)`

Call an external parsing function. The rule must only have a name and no `=` and body.

If a result type is not given, String is assumed.

An `.into()` is called on the result of the function. This is useful if a function returns an &str,
but the result type is a String.

The function signature is

```ignore
fn parse(s: &str) -> Result<(T, usize), &'static str>;
```

In case of a successful parse, a tuple with the resulting object, and the number of bytes (!) consumed
from the input string shall be returned, wrapped in Ok(). It is important that the number of bytes is
returned, not the number of characters, they will be different in the presence of utf-8 characters.
Not doing so will misalign the parser, and could also panic.

In case of an unsuccessful parse, a static string shall be returned, describing the parse problem
(preferably in "expected X" form).


Examples:

```ebnf
# Assumed return (and rule) type is String
@extern(crate::parse_identifier)
Identifier;

# Explicit return type
@extern(crate::Point::parse -> crate::Point)
Point;
```

The above will call the following functions:

```ignore
// Note that the result can be both &str or String
fn parse_identifier(s: &str) -> Result<(&str, usize), &'static str>{
    ...
}

struct Point{...}

impl Point {
    pub fn parse(s: &str) -> Result<(Self, usize), &'static str>{
        ...
    }
}
```

## Whitespace skipping

By default, peginator will skip ASCII whitespaces before every rule match, field, override, literal,
character range or end of input match.

This can be disabled on a rule basis with the `@no_skip_ws` directive.

Be sure to use `@no_skip_ws` on matched subrules too.

#### The `Whitespace` rule

The whitespace matching is just a built-in rule named `Whitespace`. It can be used just like regular
rule, which is useful for `@no_skip_ws` rules:

```ebnf
@no_skip_ws
Something = 'Sum:' Whitespace [prefix:Prefix] num:Number;
```

It can be overridden too. Comment skipping functionality can be implemented this way:

```ebnf
@no_skip_ws
Whitespace = {Comment |  '\t' | '\n' | '\x0C' | '\r' | ' '};

@no_skip_ws
Comment = '#' {!'\n' char} '\n';
```

Be sure to use `@no_skip_ws` on the `Whitespace` rule, and all rules it calls, or else the code will
most likely run into infinite recursion.
