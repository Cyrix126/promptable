# Promptable

Librairie apportant un trait et des implémentations pour différents type utilisant une librairie de prompt au choix.

La libriarie inquire permet d'avoir des prompts plus intuitifs selon les types. (par exemple date à choisir dans calendrier plutôt qu'écrire la date).

- [] inquire


## Fonctionnalité

- définit quel genre de prompt utiliser pour chaque type
- génère un message automatiquement selon le nom du champ du struct ou de la variable.
- rendre un struct promptable.
- ne demande pas lors d ela création d'un struct les champs optionnels.
- méthode pour modifier les structs avec menu interactif.
- permet des messages personalisé depuis les structs.
- implémentation personnalisable pour chaque type.

## Fonctionnement

le trait Promptable assigné à des Types permet d'avoir un prompt qui créé une valeur ou la modifie.  
Chaque type peut avoir une implémentation différente, avec des méthodes implémentés par défault grâce à des macros.  
Les structs ont une implémentation spéciale grâce à une macro dérivative. Chaque champ fait appel à la première méthode pour construire leur valeur (new_by_prompt).
Pour implémenter la seconde méthode, un menu est généré avec comme choix chaque champ. Une validation retournera le struct.

## Cas spéciaux:

Des options à la macro dérivatives sont disponibles pour:  
- ne pas afficher un champ (une valeur à la place est demandé par exemple par une fonction ou valeur par défaut du type).
- afficher un champ optionnel. Celà présente à l'utilisateur un champ à partir de la méthode new_by_prompt qui ne serait accessible uniquement dans modify_by_prompt.
- personnaliser le message du prompt pour un champ.
- afficher un nom différent qui sera repris pour le menu.

## Implémentation personnelle

Si les paramètres par défault de la macro derive ne sont pas assez précise, alors une impl Promptable for StructName est possible.