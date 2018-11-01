OuterAttr = "#" attr:Attr;
InnerAttr = "#!" attr:Attr;
Attr = "[" path:Path input:AttrInput "]";
AttrInput =
    {} |
    "=" LITERAL |
    "(" TOKEN_TREE* ")" |
    "[" TOKEN_TREE* "]" |
    "{" TOKEN_TREE* "}";
