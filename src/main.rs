use bindings::*;

fn main() -> windows::Result<()> {
    println!(
        "Component1: {}",
        runtime_component1::Class::new()?.my_property()?
    );

    println!(
        "Component2: {}",
        runtime_component2::Class::new()?.my_property()?
    );

    Ok(())
}
