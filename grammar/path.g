Path = global:"::"? segments:PathSegment* % "::";
PathSegment = ident:IDENT; // TODO generics
