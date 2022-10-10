use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "ભૂલ" => "Err",
        "બરાબર" => "Ok",
        "પંક્તિ" => "String",
        "દોરી" => "str",
        "શબ્દકોશ" => "HashMap",
        "મૂળભૂત" => "Default",
        "ચૂક" => "Error",
        "વિકલ્પ" => "Option",
        "ચોડા" => "Some",
        "કોઈ_નહિ" => "None",
        "પરિણામ" => "Result",
        "સ્વ" => "Self",
        "છાપો" => "println",
        "વિરામ" => "break",
        "અસુમેળ" => "async",
        "રાહ_જોવ" => "await",
        "પાશ" => "loop",
        "ખસેડો" => "move",
        "ડબ્બો" => "crate",
        "તરીકે" => "as",
        "સતત" => "const",
        "મેળ" => "match",
        "અસુરક્ષિત" => "unsafe",
        "માં" => "in",
        "થી" => "from",
        "લવચીક" => "dyn",
        "ખોલો" => "unwrap",
        "મૂળભૂત_" => "default",
        "અંગ" => "io",
        "બાહ્ય" => "extern",
        "ખોટું" => "false",
        "કાર્ય" => "fn",
        "મોટું" => "super",
        "અમલ" => "impl",
        "દાખલ" => "insert",
        "લક્ષણ" => "trait",
        "મેળવો" => "get",
        "ભાગ" => "mod",
        "પરિવર્તનશીલ" => "mut",
        "નવું" => "new",
        "જ્યાં" => "where",
        "માટે" => "for",
        "મેળવો_અથવા_દાખલ_કરો" => "get_or_insert_with",
        "મુખ્ય" => "main",
        "જાહેર" => "pub",
        "પરત" => "return",
        "જો" => "if",
        "નહિતો" => "else",
        "સ્વ_" => "self",
        "દો" => "let",
        "સ્થિર" => "static",
        "રચના" => "struct",
        "અપેક્ષિત" => "expect",
        "જ્યારે" => "while",
        "વાપ્રો" => "use",
        "સાચું" => "true",
        "ગણવું" => "enum",
        "અંદર" => "into",
        "સંદર્ભ" => "ref",
        "તરીકે_સંદર્ભ" => "as_ref",
        "અગમ્ય_સંહિતા" => "unreachable_code",
        "પરવાનગી આપે છે" => "મંજૂર",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn કાટ(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
