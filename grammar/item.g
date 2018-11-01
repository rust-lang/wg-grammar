ModuleContents = attrs:InnerAttr* items:ItemWithOuterAttr*;

ItemWithOuterAttr = attrs:OuterAttr* item:Item;
// TODO other items
Item =
    ExternCrate:{ "extern" "crate" name:IDENT { "as" rename:IDENT }? ";" } |
    Use:{ "use" path:Path { "as" rename:IDENT }? ";" }; // TODO use trees
