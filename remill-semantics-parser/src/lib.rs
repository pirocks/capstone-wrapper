use clang::{Entity, EntityKind};

pub fn find_isel_decls(e: &Entity, with_entity: &impl Fn(&Entity)) {
    if e.get_display_name()
        .map(|display_name|display_name.as_str().starts_with("ISEL_"))
        .unwrap_or(false) {
        with_entity(e);
    }
    for e in e.get_children() {
        find_isel_decls(&e,with_entity);
    }
}

#[cfg(test)]
mod tests {
    use clang::{Entity, EntityKind, Index};
    use crate::find_isel_decls;

    #[test]
    fn it_works() {
        let clang = clang::Clang::new().unwrap();
        let index = Index::new(&clang, false, false);
        let tu = index.parser("data/all2.cpp").parse().unwrap();

        /*let functions = tu.get_entity().get_children().into_iter().filter(|e| {
            e.get_kind() == EntityKind::UnexposedDecl
        }).flat_map(|e|e.get_children().into_iter()).collect::<Vec<_>>();*/
        find_isel_decls(&tu.get_entity(),&|e|{
            let e: &Entity = e;
            if e.get_kind() == EntityKind::VarDecl{
                dbg!(e);
                // dbg!(&e.get_template_arguments());
                dbg!(e.get_semantic_parent().unwrap().get_child(1));
            }else {
                todo!()
            }
        });
        panic!()
    }
}
