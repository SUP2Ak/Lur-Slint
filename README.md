# Lur-Slint

Un transpileur pour Slint UI permettant de générer des interfaces utilisateur dynamiquement via le langage Lur.

## Description

Lur-Slint est un module pour le langage de programmation Lur qui permet de :
- Générer dynamiquement du code Slint via une API en Rust
- Sauvegarder le code généré dans des fichiers .slint pour le debug
- Compiler directement le code généré avec l'interpréteur Slint
- Créer des applications desktop avec Slint en codant en Lur

## Installation (À venir)

Une fois que Lur sera officiellement publié, vous pourrez installer ce module via :

```bash
lur add sup2ak/lur-slint
```

## Exemple d'utilisation (Futur syntaxe Lur)

```lur
import Application from "lur-slint"

let counter: int = 0

fn button1(): void {
    counter += 1
}

fn main(): void { 
    let app = new Application({
        title: "Lur-slint",
        width: 800,
        height: 600,
        resizable: true,
        icon: "icon.png",
        properties: {
            counter: counter
        },
    })
    
    app.addButton({
        id: "button1",
        text: "Click me: {root.counter}",
        onClick: button1
    })

    app.run()
}
```

## Fonctionnalités en développement

- [ ] Système de parsing pour la mise en forme du code généré
- [ ] Support complet des composants std-widgets.slint
- [ ] Support des animations et transitions
- [ ] Gestion des variables intégrées (root, self, etc.)
- [ ] Intégration avec le compilateur Lur

## État du projet

Ce module est actuellement en développement et fait partie de l'écosystème Lur. Il sera disponible une fois que Lur sera officiellement publié.

## Contribution

Le projet est en développement actif. Les contributions seront possibles une fois le projet rendu public sur GitHub.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.