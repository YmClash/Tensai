# Tensai
 Tensaï: A Genius Tensor Language

Tensaï (天才, « génie » en japonais) est un langage de programmation orienté tenseurs, 
pensé pour la compilation et l’optimisation de calculs de Machine Learning ou d’algèbre linéaire.
Il s’appuie sur Rust pour la fiabilité et la performance, et utilise MLIR pour la génération de code
optimisé sur CPU, GPU ou d’autres cibles.

exemple de code en Tensaï: Déclaration de tenseurs
```tensai
// Tenseur 2D de shape (3, 3) avec des valeurs flottantes
tensor A = [1.0, 2.0, 3.0; 4.0, 5.0, 6.0; 7.0, 8.0, 9.0]

// Tenseur 3D de shape (2, 2, 2) avec des valeurs entières
tensor B = [[1, 2], [3, 4]; [5, 6], [7, 8]]

// Tenseur vide de shape (2, 3) avec un type explicite
tensor C: f32 = zeros(2, 3)

// Tenseur sur GPU
tensor D: f64 = ones(4, 4) @ GPU
```

exemple de code Tensai: Opérations sur les tenseurs
```tensai
// Produit tensoriel
tensor E = A @ B

// Addition élément par élément
tensor F = A + B

// Transposition
tensor G = A.T

// Contraction (somme sur certains axes)
tensor H = A #> B  // Contraction complète
tensor I = A #[0, 1]> B  // Contraction sur des axes spécifiques

// Redimensionnement
tensor J = reshape(A, (9, 1))

// Extraction de sous-tenseurs (slicing)
tensor K = A[0:2, 1:3]  // Sous-matrice de A
```

exemple de code Tensai: Fonctions mathématiques
```tensai
// Somme des éléments d'un tenseur
scalar total = sum(A)

// Moyenne des éléments
scalar avg = mean(A)

// Produit des éléments
scalar prod = product(A)

// Norme d'un tenseur
scalar norm = norm(A)

// Fonctions trigonométriques
tensor L = sin(A)
tensor M = cos(A)
tensor N = tan(A)
```

exemple de code Tensai: Boucles et contrôles de flux
```tensai
// Boucle for sur un tenseur
for i in range(A.shape[0]) {
    print(A[i, :])
}

// Condition if
if sum(A) > 100 {
    print("A est grand")
} else {
    print("A est petit")
}

// Boucle while
scalar x = 0
while x < 10 {
    x += 1
}
```

exemple de code Tensai: Fonctions et modules
```tensai
// Définition d'une fonction
fn dot_product(a: tensor, b: tensor) -> scalar {
    return sum(a * b)
}

// Appel de fonction
scalar result = dot_product(A, B)

// Importation de modules
use linalg  // Importe tout le module
use linalg::norm  // Importe une fonction spécifique
```

exemple de code Tensai: Annotations et métadonnées
```tensai
// Annotation de dispositif
@device(GPU)
tensor N = A + B

// Annotation de forme
@shape(3, 3)
tensor O = A

// Annotation de type
@dtype(f64)
tensor P = A
```

exemple de code Tensai: Programme Complet
```tensai
use linalg

// Déclaration de tenseurs
tensor A = [1.0, 2.0, 3.0; 4.0, 5.0, 6.0; 7.0, 8.0, 9.0]
tensor B = [9.0, 8.0, 7.0; 6.0, 5.0, 4.0; 3.0, 2.0, 1.0]

// Opération sur les tenseurs
tensor C = A @ B  // Produit tensoriel
tensor D = A + B  // Addition élément par élément

// Fonction personnalisée
fn compute_norm(t: tensor) -> scalar {
    return norm(t)
}

// Utilisation de la fonction
scalar norm_A = compute_norm(A)
scalar norm_B = compute_norm(B)

// Affichage des résultats
print("Norme de A :", norm_A)
print("Norme de B :", norm_B)

// Boucle pour afficher les éléments de C
for i in range(C.shape[0]) {
    print("Ligne", i, ":", C[i, :])
}

// Gestion des erreurs
assert norm_A > 0, "La norme de A doit être positive"
```

exemple de code Tensai: Syntaxe pour l'apprentissage automatique
```tensai

use nn

// Déclaration d'un modèle
model LinearRegression {
    weights: tensor,
    bias: scalar,
}

// Initialisation du modèle
fn init_model(input_size: usize) -> LinearRegression {
    return LinearRegression {
        weights: rand(input_size, 1),
        bias: 0.0,
    }
}

// Fonction de prédiction
fn predict(model: LinearRegression, x: tensor) -> tensor {
    return x @ model.weights + model.bias
}

// Entraînement du modèle
fn train(model: LinearRegression, x: tensor, y: tensor, lr: scalar, epochs: usize) {
    for epoch in range(epochs) {
        let predictions = predict(model, x)
        let error = predictions - y
        model.weights -= lr * (x.T @ error)
        model.bias -= lr * sum(error)
    }
}

// Exemple d'utilisation
let model = init_model(3)
let x = [[1.0, 2.0, 3.0]; [4.0, 5.0, 6.0]]
let y = [1.0, 2.0]
train(model, x, y, lr=0.01, epochs=100)






