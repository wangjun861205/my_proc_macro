extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

fn rc(mut items: Vec<String>, prev: String) -> String {
    if items.len() == 1 {
        return format!("({}, {})", prev, items[0]);
    }
    if prev == "" {
        let first = items.remove(0);
        return rc(items, first);
    }
    let curr = format!("({}, {})", prev, items.remove(0));
    rc(items, curr)
}

#[proc_macro]
pub fn flatten_zip(stream: TokenStream) -> TokenStream {
    let mut items = Vec::new();
    for tree in stream {
        match tree {
            TokenTree::Ident(ident) => {
                items.push(ident.to_string());
            }
            TokenTree::Punct(_) => continue,
            _ => {
                panic!("invalid group");
            }
        }
    }
    let mut res = format!("{}.into_iter()", items[0]);
    for item in items.clone().into_iter().skip(1) {
        res.push_str(&format!(".zip({})", item))
    }
    res.push_str(&format!(
        ".map(|{}| {})",
        rc(items.clone(), "".into()),
        format!("({})", items.join(","))
    ));
    res.parse().unwrap()
}
