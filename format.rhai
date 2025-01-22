// SET_INDENT is a function that is used to set the indentation string
SET_INDENT(" ");
// IND is the constant that is used to output the current indentation string
IND ++ "Rewritten ..." ++ NL;
// IND(2) is a function that is used to output the current indentation string a certain number of times
IND(2) ++ "... Tokens:" ++ NL;

let prefix = "";
for t in tokens {
    switch t.enum_type {
        // the ++ operator is used to output text
        // this is a custom DSL for formatting code in order to simplify the process of writing code formatters
        "Identifier" => prefix ++ t.value,
        "Number" => prefix ++ t.value,
        "Symbol" => prefix ++ t.value,
        "OpenParen" => prefix ++ t.value,
        "CloseParen" => prefix ++ t.value,
        "StringLiteral" => {
            prefix = "";
            switch t.quote_style {
                // NL is a constant that is used to output a newline
                "Single" => NL ++ prefix ++ "'" ++ t.value ++ "'",
                // NL() is a function that is used to output a certain number of newlines
                "Double" => NL(1) ++ prefix ++ "\"" ++ t.value ++ "\"",
            }
        },
    }
    prefix = " ";
}