use crate::{commands, Cli};

fn selection(value: Option<String>, callback: fn() -> String) -> String {
    match value {
        None => callback(),
        Some(x) => x.trim().to_string(),
    }
}

pub fn default_context(args: Cli, dest: &str) {
    let ctx = selection(args.value, || -> String {
        let contexts = commands::get_context();
        commands::selectable_list(contexts)
    });

    commands::set_default_context(&ctx);
    commands::set_context(&ctx, &dest);
}

pub fn context(args: Cli, dest: &str) {
    let ctx = selection(args.value, || -> String {
        let contexts = commands::get_context();
        commands::selectable_list(contexts)
    });

    commands::set_context(&ctx, &dest);

    println!("{}/{}", &dest, str::replace(&ctx, ":", "_"));
}

pub fn namespace(args: Cli, dest: &str) {
    let ctx = commands::get_current_context();
    let ns = selection(args.value, || -> String {
        let namespaces = commands::get_namespaces();
        commands::selectable_list(namespaces)
    });

    commands::set_namespace(&ctx, &ns, &dest);

    println!("{}/{}", &dest, str::replace(&ctx, ":", "_"));
}

pub fn default_namespace(args: Cli, dest: &str) {
    let ctx = commands::get_current_context();
    let ns = selection(args.value, || -> String {
        let namespaces = commands::get_namespaces();
        commands::selectable_list(namespaces)
        });

    commands::set_default_namespace(&ns);
    commands::set_namespace(&ctx, &ns, &dest);
}
