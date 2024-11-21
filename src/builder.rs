use slint_interpreter::{Compiler,/* ComponentDefinition, ComponentHandle, SharedString, Value,*/ ComponentInstance};
use std::fs;
use std::path::Path;

pub struct SlintTranspiler {
    components: Vec<Component>,
}

struct Component {
    name: String,
    properties: Vec<Property>,
    callbacks: Vec<Callback>,
    layout: Layout,
}

struct Property {
    name: String,
    type_name: String,
    default_value: String,
}

struct Callback {
    name: String,
    params: Vec<String>,
}

struct Layout {
    elements: Vec<Element>,
}

enum Element {
    Button { text: String, callback: String },
    // Autres items à venir
}

impl SlintTranspiler {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, name: &str) -> &mut Self {
        self.components.push(Component {
            name: name.to_string(),
            properties: Vec::new(),
            callbacks: Vec::new(),
            layout: Layout {
                elements: Vec::new(),
            },
        });
        self
    }

    pub fn add_property(&mut self, name: &str, type_name: &str, default_value: &str) -> &mut Self {
        self.components
            .last_mut()
            .unwrap()
            .properties
            .push(Property {
                name: name.to_string(),
                type_name: type_name.to_string(),
                default_value: default_value.to_string(),
            });
        self
    }

    pub fn add_callback(&mut self, name: &str, params: Vec<&str>) -> &mut Self {
        self.components
            .last_mut()
            .unwrap()
            .callbacks
            .push(Callback {
                name: name.to_string(),
                params: params.iter().map(|s| s.to_string()).collect(),
            });
        self
    }

    pub fn add_button(&mut self, text: &str, callback: &str) -> &mut Self {
        self.components
            .last_mut()
            .unwrap()
            .layout
            .elements
            .push(Element::Button {
                text: text.to_string(),
                callback: callback.to_string(),
            });
        self
    }

    /*
        Générer le contenu Slint Dynamiquement
        Sauvegarder le contenue généré dans un fichier .slint
        Compiler avec l'interpréteur
        Retourner l'instance du composant

        Il reste à ajouter la gestion des erreurs
        Ainsi qu'un systeme de parsing, pour les tabulations, les retours à la ligne, etc. selon les contextes
        Ainsi qu'intégrer tout les composants std-widgets.slint (VerticalBox, Button, etc.)
        Ainsi que la possibilité d'ajouter des éléments supplémentaires dans les layouts comme les animations, etc.
        Plus parser correctement les variables intégrées dans les propriétés, comme root.counter, self.text, etc... avec le "\" devant "{root}"
     */
    pub fn build(&self) -> Result<ComponentInstance, Box<dyn std::error::Error>> {
        // Générer le contenu Slint
        let mut content = String::from("import { Button, VerticalBox } from \"std-widgets.slint\";\n\n");
        
        for component in &self.components {
            content.push_str(&format!("export component {} inherits Window {{\n", component.name));
            
            // Génération des propriétés
            for prop in &component.properties {
                content.push_str(&format!("    in-out property <{}> {}: {};\n", 
                    prop.type_name, prop.name, prop.default_value));
            }
            
            // Génération des callbacks
            for callback in &component.callbacks {
                content.push_str(&format!("    callback {}({});\n", 
                    callback.name, callback.params.join(", ")));
            }
            
            // Génération du layout
            content.push_str("    VerticalBox {\n");
            for element in &component.layout.elements {
                match element {
                    Element::Button { text, callback } => {
                        content.push_str(&format!("        Button {{ text: \"{}\"; clicked => {{ {} }} }}\n", 
                            text, callback));
                    }
                }
            }
            content.push_str("    }\n}\n");
        }
    
        // Debug: sauvegarder le contenue généré dans un fichier .slint
        let path = Path::new("ui/generated.slint");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &content)?;
    
        // Compiler avec l'interpréteur
        let compiler = Compiler::default();
        let result = spin_on::spin_on(compiler.build_from_source(content, Default::default()));
        let definition = result.component("MainWindow").ok_or("Component not found")?;
        Ok(definition.create()?)
    }
}
