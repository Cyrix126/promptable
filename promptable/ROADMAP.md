# ROADMAP



## Utiliser des fonctions custom pour chaque field


Pour éviter que le progammeur ait à réimplémenter pour chaque cas spéciaux et à déclarer des connexions nouvelles à l'intérieur des fonction

On oublie les traits pour créer uniquement des implémentations qui peuvent être différente. Celà permet d'avoir des paramètres différent.
On a alors un attribut qui permet de définir la signature de la méthode.
Et un attribut par field pour préciser une fonction à utiliser pour créer la valeur. Cette fonction peut alors utiliser le paramètre écrit dans la méthode.

Pour les fields, la fonction utilisé pour générer la valeur peut avoir trois valeurs:

- rien de préciser, utiliser la fonction new_by_prompt et modify_by_prompt ou multiple_new_by_prompt pour vec.

- si type est Option, utiliser Some(inner type new_by_prompt).

- default précisé, le type doit implémenter le type Default et la valeur de défaut sera utilisé.

- fonction avec ses paramètres précisé. Le nom d'une fonction avec les noms de variables en paramètres à l'identique des noms utilisé pour les paramètres de la méthode indiqué dans l'attribut params de la structure.

- None pour attribut invisible ou is_option

Le modify_by_prompt contiendra toujours (&mut self).

Si cette technique de création de paramètres aux méthodes est utilisé, on peux quand même utiliser un trait pour les types qui n'utilisent pas cette macro.

L'attribut params est décortiqué pour pouvoir les replacer dans les arguments de fonctions qui ont besoin de les faire passer.


## Mutltiple création/modification

Plutôt qu'une méhode pour créer un struct et une autre pour le modifier.
Une méthode pour créer plusieurs structs et pouvoir chacun les modifier.