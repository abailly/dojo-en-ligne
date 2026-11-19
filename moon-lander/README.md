# Prérequis
- Se générer une clé SSH et la communiquer à la personne qui gère le serveur
- Récupérer l'IP du serveur
- Lancer ssh-keygen -R [serveur_ip]
- Lancer ssh -A curry@serveur_ip


# Modèle physique et représentation de l'état

L'état du vaisseau à un instant t est représenté par une structure immuable (souvent une liste ou une paire) contenant trois valeurs :

- Altitude (h) : hauteur au-dessus de la surface lunaire (en mètres).
- Vitesse (v) : vitesse verticale (en m/s, négative lors de la descente).
- Carburant (f) : quantité de carburant restante (en kg ou unités arbitraires).

Équations de mise à jour (pour un pas de temps Δ t)

À chaque étape, le pilote ou l'algorithme de contrôle choisit une poussée (taux de combustion b, borné par le carburant disponible et la poussée maximale du moteur) :

- Carburant consommé : \(\text{burn} = \min(b, f / \Delta t)\)
- Nouveau carburant : $f' = f - burn ⋅ Δ t$
- Accélération nette : $a = (force_poussée × burn) - g$ (avec la gravité lunaire g ≈ 1.62 m/s²)
- Nouvelle vitesse : $v' = v + a ⋅ Δ t$
- Nouvelle altitude : \(h' = h + v \cdot \Delta t + \frac{1}{2} a \cdot \Delta t^2\) (ou l'approximation d'Euler $h' = h + v' ⋅ Δ t$)



Constantes :

- poussée moteur = 1.0
- altitude initiale = 50.0
- vélocité initiale = 0.0
