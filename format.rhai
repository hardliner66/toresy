let prefix = "";
for t in tokens {
    switch t.enum_type {
        "Identifier" => prefix ++ t.value,
        "Number" => prefix ++ t.value,
        "Symbol" => prefix ++ t.value,
        "OpenParen" => prefix ++ t.value,
        "CloseParen" => prefix ++ t.value,
        "StringLiteral" => switch t.quote_style {
            "Single" => prefix ++ "'" ++ t.value ++ "'",
            "Double" => prefix ++ "\"" ++ t.value ++ "\"",
        },
    }
    prefix = " ";
}