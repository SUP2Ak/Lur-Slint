/* Lur-slint
    Ceci est l'initiateur du projet Lur-slint

    Lur-slint est un transpiler pour Slint, un langage de création d'interface utilisateur.
    Il permet de générer dynamiquement du code Slint, de le sauvegarder dans un fichier .slint, de le compiler avec l'interpréteur Slint, et de récupérer l'instance du composant.
    Ceci sera le module qu'on pourra importer via le cli de Lur. exemple (lur add sup2ak/lur-slint)
    Ainsi, on pourra générer du code Slint, le sauvegarder, le compiler, et l'utiliser dans notre projet Lur.

    Vu que Lur n'est pas encore publié officiellement sur github, il manque l'intégration de Lur via cargo
    Donc par exemple dans le cargo.toml ici dans les dépendances, il faut ajouter :
    - lur = "0.1.0"

    Pour ensuite wrapper notre code Rust via l'api de Lur.
    LurApi::new()
*/

mod builder;
use builder::SlintTranspiler;
use slint_interpreter::{ComponentHandle, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut transpiler = SlintTranspiler::new();
    
    transpiler
        .add_component("MainWindow")
        .add_property("counter", "int", "0")
        .add_property("button-texts", "[string]", "[]")
        .add_callback("button-clicked", vec!["string"])
        .add_button("Premier Bouton: {root.counter}", "button-clicked(root.counter + 1)")
        .add_button("Deuxième Bouton", "button-clicked(self.text)");

    let window = transpiler.build()?;
    
    let _ =window.set_callback("button-clicked", |args| {
        if let &[Value::String(ref text)] = args {
            println!("Button clicked: {}", text);
            Value::Void
        } else {
            Value::Void
        }
    });

    window.run()?;
    Ok(())
}